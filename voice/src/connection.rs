use crate::error::VoiceError;
use livekit::options::TrackPublishOptions;
use livekit::track::LocalAudioTrack;
use livekit::webrtc::audio_frame::AudioFrame;
use livekit::webrtc::audio_source::native::NativeAudioSource;
use livekit::{ConnectionState, Room, RoomOptions};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify, mpsc};

const FRAME_SIZE: usize = 480;
const SAMPLE_RATE: u32 = 48000;

pub struct FluxerVoiceConnection {
    pub guild_id: String,
    pub channel_id: String,
    pub connection_id: String,
    room: Arc<Room>,
    audio_source: NativeAudioSource,
    playback_lock: Arc<Mutex<()>>,
    stop_signal: Arc<Notify>,
    track_published: Arc<Mutex<bool>>,
}

impl FluxerVoiceConnection {
    pub async fn connect(
        endpoint: &str,
        token: &str,
        guild_id: String,
        channel_id: String,
        connection_id: String,
    ) -> Result<Arc<Self>, VoiceError> {
        let (room, _) = Room::connect(endpoint, token, RoomOptions::default()).await?;

        let audio_source = NativeAudioSource::new(
            livekit::webrtc::audio_source::AudioSourceOptions::default(),
            SAMPLE_RATE,
            1,
            FRAME_SIZE as u32,
        );

        Ok(Arc::new(Self {
            guild_id,
            channel_id,
            connection_id,
            room: Arc::new(room),
            audio_source,
            playback_lock: Arc::new(Mutex::new(())),
            stop_signal: Arc::new(Notify::new()),
            track_published: Arc::new(Mutex::new(false)),
        }))
    }

    async fn ensure_track_published(&self) -> Result<(), VoiceError> {
        let mut published = self.track_published.lock().await;
        if *published {
            return Ok(());
        }

        tracing::info!("Publishing audio track...");
        let track = LocalAudioTrack::create_audio_track(
            "audio",
            livekit::webrtc::audio_source::RtcAudioSource::Native(self.audio_source.clone()),
        );
        let opts = TrackPublishOptions {
            source: livekit::track::TrackSource::Microphone,
            ..Default::default()
        };
        self.room
            .local_participant()
            .publish_track(livekit::track::LocalTrack::Audio(track), opts)
            .await?;

        *published = true;
        tracing::info!("Audio track published");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.room.connection_state() == ConnectionState::Connected
    }

    pub async fn stop(&self) -> Result<(), VoiceError> {
        self.stop_signal.notify_waiters();
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), VoiceError> {
        self.stop().await?;
        let _ = self.room.close().await;
        Ok(())
    }

    pub async fn play_file(&self, path: &str) -> Result<(), VoiceError> {
        let _lock = self.playback_lock.lock().await;

        let path = path.to_string();
        let mut hint = symphonia::core::probe::Hint::new();
        if let Some(ext) = std::path::Path::new(&path)
            .extension()
            .and_then(|e| e.to_str())
        {
            hint.with_extension(ext);
        }

        let (tx, rx) = mpsc::channel::<Vec<i16>>(32);

        let decode_handle = tokio::task::spawn_blocking(move || {
            let file =
                std::fs::File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
            let mss = Box::new(file) as Box<dyn symphonia::core::io::MediaSource>;
            crate::pcm::decode_streaming(mss, &hint, tx)
        });

        self.ensure_track_published().await?;

        self.stream_pcm_channel(rx).await?;

        decode_handle
            .await
            .map_err(|e| VoiceError::PlaybackError(e.to_string()))?
            .map_err(VoiceError::PlaybackError)?;

        Ok(())
    }

    pub async fn play_bytes(&self, data: Vec<u8>) -> Result<(), VoiceError> {
        let _lock = self.playback_lock.lock().await;

        let hint = symphonia::core::probe::Hint::new();
        let (tx, rx) = mpsc::channel::<Vec<i16>>(32);

        let decode_handle = tokio::task::spawn_blocking(move || {
            let cursor = std::io::Cursor::new(data);
            let mss = Box::new(cursor) as Box<dyn symphonia::core::io::MediaSource>;
            crate::pcm::decode_streaming(mss, &hint, tx)
        });

        self.ensure_track_published().await?;
        self.stream_pcm_channel(rx).await?;

        decode_handle
            .await
            .map_err(|e| VoiceError::PlaybackError(e.to_string()))?
            .map_err(VoiceError::PlaybackError)?;

        Ok(())
    }

    async fn stream_pcm_channel(&self, mut rx: mpsc::Receiver<Vec<i16>>) -> Result<(), VoiceError> {
        let mut buffer: Vec<i16> = Vec::with_capacity(FRAME_SIZE * 4);

        'outer: loop {
            match rx.recv().await {
                Some(chunk) => buffer.extend_from_slice(&chunk),
                None => {
                    if !buffer.is_empty() {
                        buffer.resize(FRAME_SIZE, 0);
                        self.send_frame(&buffer[..FRAME_SIZE]).await;
                    }
                    break;
                }
            }

            while buffer.len() >= FRAME_SIZE {
                let stopped = tokio::select! {
                    biased;
                    _ = self.stop_signal.notified() => true,
                    _ = std::future::ready(()) => false,
                };
                if stopped {
                    break 'outer;
                }
                self.send_frame(&buffer[..FRAME_SIZE]).await;
                buffer.drain(..FRAME_SIZE);
            }
        }
        Ok(())
    }

    async fn send_frame(&self, samples: &[i16]) {
        let mut data = samples.to_vec();
        if data.len() < FRAME_SIZE {
            data.resize(FRAME_SIZE, 0);
        }
        let frame = AudioFrame {
            data: data.into(),
            sample_rate: SAMPLE_RATE,
            num_channels: 1,
            samples_per_channel: FRAME_SIZE as u32,
        };
        let _ = self.audio_source.capture_frame(&frame).await;
    }
}
