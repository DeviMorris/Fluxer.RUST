use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use tokio::sync::mpsc;

const TARGET_SAMPLE_RATE: u32 = 48000;

pub fn decode_streaming(
    source: Box<dyn MediaSource>,
    hint: &Hint,
    tx: mpsc::Sender<Vec<i16>>,
) -> Result<(), String> {
    let mss = MediaSourceStream::new(source, Default::default());

    let probed = symphonia::default::get_probe()
        .format(
            hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("Failed to probe format: {:?}", e))?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| "No supported audio track found".to_string())?;

    let track_id = track.id;
    let source_sample_rate = track.codec_params.sample_rate.unwrap_or(TARGET_SAMPLE_RATE);
    let needs_resample = source_sample_rate != TARGET_SAMPLE_RATE;
    let resample_ratio = source_sample_rate as f64 / TARGET_SAMPLE_RATE as f64;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())
        .map_err(|e| format!("Failed to create decoder: {:?}", e))?;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(SymphoniaError::IoError(e)) => {
                let msg = e.to_string();
                if msg.contains("end of file") || msg.contains("end of stream") {
                    break;
                }
                return Err(format!("IO Error: {:?}", e));
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(_) => break,
        };

        if packet.track_id() != track_id {
            continue;
        }

        let audio_buf = match decoder.decode(&packet) {
            Ok(buf) => buf,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("Decode error: {:?}", e)),
        };

        let mut sample_buf =
            SampleBuffer::<i16>::new(audio_buf.capacity() as u64, *audio_buf.spec());
        let channels = audio_buf.spec().channels.count();
        sample_buf.copy_interleaved_ref(audio_buf);
        let samples = sample_buf.samples();

        let mono: Vec<i16> = if channels > 1 {
            samples
                .chunks_exact(channels)
                .map(|chunk| {
                    let sum: i32 = chunk.iter().map(|&s| s as i32).sum();
                    (sum / channels as i32) as i16
                })
                .collect()
        } else {
            samples.to_vec()
        };

        let chunk = if needs_resample {
            let new_len = (mono.len() as f64 / resample_ratio).ceil() as usize;
            (0..new_len)
                .map(|i| {
                    let src_idx = i as f64 * resample_ratio;
                    let idx1 = src_idx.floor() as usize;
                    let idx2 = (idx1 + 1).min(mono.len().saturating_sub(1));
                    let frac = src_idx - idx1 as f64;
                    let val = mono[idx1] as f64 + (mono[idx2] as f64 - mono[idx1] as f64) * frac;
                    val as i16
                })
                .collect()
        } else {
            mono
        };

        if tx.blocking_send(chunk).is_err() {
            break;
        }
    }

    Ok(())
}
