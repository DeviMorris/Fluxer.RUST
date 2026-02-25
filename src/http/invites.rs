use super::channels::InviteResponse;
use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};

#[derive(Debug, Clone)]
pub struct InvitesApi {
    http: HttpClient,
}

impl InvitesApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_invite(&self, invite_code: &str) -> Result<InviteResponse> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/invites/{invite.code}")
            .compile(&QueryValues::new(), &[("invite.code", invite_code)])?;
        self.http
            .request_json::<(), InviteResponse>(&ep, None)
            .await
    }

    pub async fn accept_invite(&self, invite_code: &str) -> Result<InviteResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/invites/{invite.code}")
            .compile(&QueryValues::new(), &[("invite.code", invite_code)])?;
        self.http
            .request_json::<(), InviteResponse>(&ep, None)
            .await
    }

    pub async fn delete_invite(&self, invite_code: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/invites/{invite.code}")
            .compile(&QueryValues::new(), &[("invite.code", invite_code)])?;
        self.http.request_unit::<()>(&ep, None).await
    }
}
