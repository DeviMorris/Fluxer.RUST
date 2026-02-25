use crate::error::Result;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct AdminApi {
    http: HttpClient,
}

impl AdminApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn add_email_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/email/add")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn add_ip_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/ip/add")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn add_phone_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/phone/add")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn add_snowflake_reservation(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/snowflake-reservations/add")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn admin_resend_verification_email(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/resend-verification-email")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn approve_discovery_application(&self, guild_id: &str, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/discovery/applications/{guild_id}/approve")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild_id", guild_id),
                ],
            )?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn approve_system_dm_job(&self, job_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/system-dm-jobs/{job_id}/approve")
            .compile(
                &QueryValues::new(),
                &[
                    ("job_id", job_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn ban_guild_member(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/ban-member")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn bulk_add_guild_members(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bulk/add-guild-members")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn bulk_update_guild_features(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bulk/update-guild-features")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn bulk_update_user_flags(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bulk/update-user-flags")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn cancel_account_deletion(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/cancel-deletion")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn cancel_bulk_message_deletion(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/cancel-bulk-message-deletion")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn change_user_dob(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/change-dob")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn change_user_email(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/change-email")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn change_user_username(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/change-username")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn check_email_ban_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/email/check")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn check_ip_ban_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/ip/check")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn check_phone_ban_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/phone/check")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn clear_guild_fields(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/clear-fields")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn clear_user_fields(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/clear-fields")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn create_admin_api_key(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/api-keys")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn create_system_dm_job(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/system-dm-jobs")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn create_voice_region(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/regions/create")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn create_voice_server(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/servers/create")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_admin_api_key(&self, key_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Delete, "/admin/api-keys/{keyId}")
            .compile(
                &QueryValues::new(),
                &[
                    ("keyId", key_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn delete_all_user_messages(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/delete-all")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_guild(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_message(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_snowflake_reservation(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/snowflake-reservations/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_user_webauthn_credential(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/delete-webauthn-credential")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_voice_region(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/regions/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn delete_voice_server(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/servers/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn disable_user_mfa(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/disable-mfa")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn disable_user_suspicious(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/disable-suspicious")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn expand_visionary_slots(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/visionary-slots/expand")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn force_add_user_to_guild(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/force-add-user")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn generate_gift_subscription_codes(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/codes/gift")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_archive_details(&self, subject_type: &str, subject_id: &str, archive_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/archives/{subjectType}/{subjectId}/{archiveId}")
            .compile(
                &QueryValues::new(),
                &[
                    ("subjectType", subject_type),
                    ("subjectId", subject_id),
                    ("archiveId", archive_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_archive_download_url(&self, subject_type: &str, subject_id: &str, archive_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/archives/{subjectType}/{subjectId}/{archiveId}/download")
            .compile(
                &QueryValues::new(),
                &[
                    ("subjectType", subject_type),
                    ("subjectId", subject_id),
                    ("archiveId", archive_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_authenticated_admin_user(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/users/me")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_gateway_node_statistics(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/gateway/stats")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_guild_memory_statistics(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/gateway/memory-stats")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_instance_config(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/instance-config/get")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_legal_hold_status(&self, report_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/reports/{report_id}/legal-hold")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_limit_config(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/limit-config/get")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_message_shred_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/shred-status")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_ncmec_submission_status(&self, report_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/reports/{report_id}/ncmec-status")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_report(&self, report_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/reports/{report_id}")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_search_index_refresh_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/search/refresh-status")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_user_change_log(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/change-log")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_voice_region(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/regions/get")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn get_voice_server(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/servers/get")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn kick_guild_member(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/kick-member")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_admin_api_keys(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/api-keys")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_archives(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/archives/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_audit_logs(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/audit-logs")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_discovery_applications(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/discovery/applications")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_email_bans(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/email/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_guild_emojis(&self, guild_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/guilds/{guild_id}/emojis")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild_id", guild_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_guild_members(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/list-members")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_guild_stickers(&self, guild_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/guilds/{guild_id}/stickers")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild_id", guild_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_ip_bans(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/ip/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_phone_bans(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/phone/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_reports(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/reports/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_snowflake_reservations(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/snowflake-reservations/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_system_dm_jobs(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/system-dm-jobs")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_user_dm_channels(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/list-dm-channels")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_user_guilds(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/list-guilds")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_user_sessions(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/list-sessions")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_user_webauthn_credentials(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/list-webauthn-credentials")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_visionary_slots(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/admin/visionary-slots")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn list_voice_regions(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/regions/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn list_voice_servers(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/servers/list")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn lookup_guild(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/lookup")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn lookup_message(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/lookup")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn lookup_message_by_attachment(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/lookup-by-attachment")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn lookup_user(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/lookup")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn purge_guild_assets(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/assets/purge")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn queue_message_shred(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/messages/shred")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn refresh_search_index(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/search/refresh-index")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn reject_discovery_application(&self, guild_id: &str, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/discovery/applications/{guild_id}/reject")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild_id", guild_id),
                ],
            )?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn release_legal_hold_on_evidence(&self, report_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Delete, "/admin/reports/{report_id}/legal-hold")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn reload_all_specified_guilds(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/gateway/reload-all")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn reload_guild(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/reload")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn remove_email_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/email/remove")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn remove_from_discovery(&self, guild_id: &str, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/discovery/guilds/{guild_id}/remove")
            .compile(
                &QueryValues::new(),
                &[
                    ("guild_id", guild_id),
                ],
            )?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn remove_ip_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/ip/remove")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn remove_phone_ban(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bans/phone/remove")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn reserve_visionary_slot(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/visionary-slots/reserve")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn resolve_report(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/reports/resolve")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn schedule_account_deletion(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/schedule-deletion")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn schedule_bulk_user_deletion(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/bulk/schedule-user-deletion")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn search_audit_logs(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/audit-logs/search")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn search_guilds(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/search")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn search_reports(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/reports/search")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn search_users(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/search")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn send_password_reset(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/send-password-reset")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn set_legal_hold_on_evidence(&self, report_id: &str, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/reports/{report_id}/legal-hold")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn set_user_acls(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/set-acls")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn set_user_bot_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/set-bot-status")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn set_user_system_status(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/set-system-status")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn set_user_traits(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/set-traits")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn shrink_visionary_slots(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/visionary-slots/shrink")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn shutdown_guild(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/shutdown")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn submit_report_to_ncmec(&self, report_id: &str) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/reports/{report_id}/ncmec-submit")
            .compile(
                &QueryValues::new(),
                &[
                    ("report_id", report_id),
                ],
            )?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn swap_visionary_slots(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/visionary-slots/swap")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn temp_ban_user(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/temp-ban")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn terminate_user_sessions(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/terminate-sessions")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn transfer_guild_ownership(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/transfer-ownership")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn trigger_guild_archive(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/archives/guild")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn trigger_user_archive(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/archives/user")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn unban_user(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/unban")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn unlink_user_phone(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/unlink-phone")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_guild_features(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/update-features")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_guild_name(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/update-name")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_guild_settings(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/update-settings")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_guild_vanity(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/guilds/update-vanity")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_instance_config(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/instance-config/update")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_limit_config(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/limit-config/update")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_suspicious_activity_flags(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/update-suspicious-activity-flags")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_user_flags(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/update-flags")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_voice_region(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/regions/update")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn update_voice_server(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/voice/servers/update")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

    pub async fn verify_user_email(&self, body: &AdminRequest) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Post, "/admin/users/verify-email")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<AdminRequest, Value>(&ep, Some(body)).await
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}
