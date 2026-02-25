use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct PacksApi {
    http: HttpClient,
}

impl PacksApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn list_packs(&self) -> Result<Vec<PackResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/packs").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<PackResponse>>(&ep, None)
            .await
    }

    pub async fn list_pack_emojis(&self, pack_id: &str) -> Result<Vec<PackItemResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/packs/emojis/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<(), Vec<PackItemResponse>>(&ep, None)
            .await
    }

    pub async fn create_pack_emoji(&self, pack_id: &str, body: &PackItemRequest) -> Result<PackItemResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/emojis/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackItemRequest, PackItemResponse>(&ep, Some(body))
            .await
    }

    pub async fn bulk_create_pack_emojis(
        &self,
        pack_id: &str,
        body: &PackBulkRequest,
    ) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/emojis/{packId}/bulk")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackBulkRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn update_pack_emoji(
        &self,
        pack_id: &str,
        emoji_id: &str,
        body: &PackItemRequest,
    ) -> Result<PackItemResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/packs/emojis/{packId}/{emojiId}")
            .compile(
                &QueryValues::new(),
                &[("packId", pack_id), ("emojiId", emoji_id)],
            )?;
        self.http
            .request_json::<PackItemRequest, PackItemResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_pack_emoji(&self, pack_id: &str, emoji_id: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/packs/emojis/{packId}/{emojiId}")
            .compile(
                &QueryValues::new(),
                &[("packId", pack_id), ("emojiId", emoji_id)],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_pack_stickers(&self, pack_id: &str) -> Result<Vec<PackItemResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/packs/stickers/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<(), Vec<PackItemResponse>>(&ep, None)
            .await
    }

    pub async fn create_pack_sticker(&self, pack_id: &str, body: &PackItemRequest) -> Result<PackItemResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/stickers/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackItemRequest, PackItemResponse>(&ep, Some(body))
            .await
    }

    pub async fn bulk_create_pack_stickers(
        &self,
        pack_id: &str,
        body: &PackBulkRequest,
    ) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/stickers/{packId}/bulk")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackBulkRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn update_pack_sticker(
        &self,
        pack_id: &str,
        sticker_id: &str,
        body: &PackItemRequest,
    ) -> Result<PackItemResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/packs/stickers/{packId}/{stickerId}")
            .compile(
                &QueryValues::new(),
                &[("packId", pack_id), ("stickerId", sticker_id)],
            )?;
        self.http
            .request_json::<PackItemRequest, PackItemResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_pack_sticker(&self, pack_id: &str, sticker_id: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/packs/stickers/{packId}/{stickerId}")
            .compile(
                &QueryValues::new(),
                &[("packId", pack_id), ("stickerId", sticker_id)],
            )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn create_pack(&self, pack_type: &str, body: &PackRequest) -> Result<PackResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/{packType}")
            .compile(&QueryValues::new(), &[("packType", pack_type)])?;
        self.http
            .request_json::<PackRequest, PackResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_pack(&self, pack_id: &str, body: &PackRequest) -> Result<PackResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/packs/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackRequest, PackResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_pack(&self, pack_id: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/packs/{packId}")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_pack_invites(&self, pack_id: &str) -> Result<Vec<Value>> {
        let ep = Endpoint::new(HttpMethod::Get, "/packs/{packId}/invites")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http.request_json::<(), Vec<Value>>(&ep, None).await
    }

    pub async fn create_pack_invite(&self, pack_id: &str, body: &PackInviteRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/{packId}/invites")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackInviteRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn install_pack(&self, pack_id: &str, body: &PackInstallRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/packs/{packId}/install")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http
            .request_json::<PackInstallRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn uninstall_pack(&self, pack_id: &str) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/packs/{packId}/install")
            .compile(&QueryValues::new(), &[("packId", pack_id)])?;
        self.http.request_unit::<()>(&ep, None).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackItemResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackItemRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackBulkRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackInviteRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackInstallRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
