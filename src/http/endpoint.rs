use crate::error::{ProtocolError, Result};
use std::collections::BTreeMap;
use url::form_urlencoded;

pub const API_BASE: &str = "https://api.fluxer.app/";
pub const MAJOR_PARAMETERS: [&str; 4] =
    ["guild.id", "channel.id", "webhook.id", "interaction.token"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthPolicy {
    Bot,
    NoBot,
}

impl AuthPolicy {
    pub const fn requires_bot(self) -> bool {
        matches!(self, Self::Bot)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub method: HttpMethod,
    pub route: &'static str,
    pub auth: AuthPolicy,
}

impl Endpoint {
    pub const fn new(method: HttpMethod, route: &'static str) -> Self {
        Self {
            method,
            route,
            auth: AuthPolicy::Bot,
        }
    }

    pub const fn new_no_bot_auth(method: HttpMethod, route: &'static str) -> Self {
        Self {
            method,
            route,
            auth: AuthPolicy::NoBot,
        }
    }

    pub fn compile(
        &self,
        query: &QueryValues,
        params: &[(&str, &str)],
    ) -> Result<CompiledEndpoint> {
        let (path, major_params) = compile_route(self.route, params)?;
        let query_encoded = query.encode();
        let url = if query_encoded.is_empty() {
            path.clone()
        } else {
            format!("{path}?{query_encoded}")
        };

        Ok(CompiledEndpoint {
            method: self.method,
            route: self.route,
            auth: self.auth,
            path,
            url,
            major_params,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledEndpoint {
    pub method: HttpMethod,
    pub route: &'static str,
    pub auth: AuthPolicy,
    pub path: String,
    pub url: String,
    pub major_params: String,
}

impl CompiledEndpoint {
    pub fn rate_limit_key(&self, bucket_hash: &str) -> String {
        if self.major_params.is_empty() {
            bucket_hash.to_owned()
        } else {
            format!("{bucket_hash}:{}", self.major_params)
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueryValues(BTreeMap<String, String>);

impl QueryValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ToString,
    {
        self.0.insert(key.into(), value.to_string());
    }

    pub fn insert_opt<K, V>(&mut self, key: K, value: Option<V>)
    where
        K: Into<String>,
        V: ToString,
    {
        if let Some(value) = value {
            self.0.insert(key.into(), value.to_string());
        }
    }

    pub fn encode(&self) -> String {
        let mut serializer = form_urlencoded::Serializer::new(String::new());
        for (key, value) in &self.0 {
            serializer.append_pair(key, value);
        }
        serializer.finish()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn compile_route(route: &str, params: &[(&str, &str)]) -> Result<(String, String)> {
    let mut path = String::with_capacity(route.len() + 32);
    let mut major = Vec::new();
    let mut idx = 0;

    while let Some(start_rel) = route[idx..].find('{') {
        let start = idx + start_rel;
        path.push_str(&route[idx..start]);

        let end_rel = route[start + 1..]
            .find('}')
            .ok_or_else(|| ProtocolError::InvalidRouteTemplate(route.to_owned()))?;
        let end = start + 1 + end_rel;
        let key = &route[start + 1..end];

        let value = find_param(params, key)
            .ok_or_else(|| ProtocolError::MissingRouteParam(key.to_owned()))?;

        path.push_str(value);
        if is_major_param(key) {
            major.push(format!("{key}={value}"));
        }

        idx = end + 1;
    }

    if route[idx..].contains('}') {
        return Err(ProtocolError::InvalidRouteTemplate(route.to_owned()).into());
    }

    path.push_str(&route[idx..]);
    Ok((path, major.join(":")))
}

fn find_param<'a>(params: &'a [(&str, &'a str)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find_map(|(param_key, value)| (*param_key == key).then_some(*value))
}

fn is_major_param(param: &str) -> bool {
    MAJOR_PARAMETERS.contains(&param)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_path_query_major() {
        let ep = Endpoint::new(
            HttpMethod::Get,
            "/channels/{channel.id}/messages/{message.id}",
        );
        let mut query = QueryValues::new();
        query.insert("limit", 100);
        query.insert("around", "123");

        let compiled = ep
            .compile(
                &query,
                &[
                    ("channel.id", "42"),
                    ("message.id", "999"),
                    ("guild.id", "77"),
                ],
            )
            .expect("compile endpoint");

        assert_eq!(compiled.path, "/channels/42/messages/999");
        assert_eq!(
            compiled.url,
            "/channels/42/messages/999?around=123&limit=100"
        );
        assert_eq!(compiled.major_params, "channel.id=42");
        assert!(compiled.auth.requires_bot());
    }

    #[test]
    fn major_params_order() {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}/webhooks/{webhook.id}");
        let query = QueryValues::new();

        let compiled = ep
            .compile(&query, &[("webhook.id", "55"), ("guild.id", "1")])
            .expect("compile endpoint");

        assert_eq!(compiled.major_params, "guild.id=1:webhook.id=55");
    }

    #[test]
    fn missing_param_error() {
        let ep = Endpoint::new(HttpMethod::Get, "/guilds/{guild.id}");
        let query = QueryValues::new();
        let err = ep.compile(&query, &[]).expect_err("missing param");

        match err {
            crate::error::Error::Protocol(ProtocolError::MissingRouteParam(param)) => {
                assert_eq!(param, "guild.id");
            }
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn no_bot_auth() {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Post, "/oauth2/token");
        let query = QueryValues::new();
        let compiled = ep.compile(&query, &[]).expect("compile endpoint");

        assert!(!compiled.auth.requires_bot());
    }

    #[test]
    fn query_encoding() {
        let mut query = QueryValues::new();
        query.insert("query", "hello world");
        query.insert("emoji", "a+b/c");

        let encoded = query.encode();
        assert_eq!(encoded, "emoji=a%2Bb%2Fc&query=hello+world");
    }
}
