use super::channels::ChannelResponse;
use crate::error::Result;
use crate::flags::UserFlags;
use crate::http::{Endpoint, HttpClient, HttpMethod, QueryValues};
use crate::id::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct UsersApi {
    http: HttpClient,
}

impl UsersApi {
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn get_me(&self) -> Result<UserPrivateResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), UserPrivateResponse>(&ep, None)
            .await
    }

    pub async fn update_me(&self, body: &UserProfileUpdateRequest) -> Result<UserPrivateResponse> {
        let ep =
            Endpoint::new(HttpMethod::Patch, "/users/@me").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<UserProfileUpdateRequest, UserPrivateResponse>(&ep, Some(body))
            .await
    }

    pub async fn get_my_channels(&self) -> Result<Vec<ChannelResponse>> {
        let ep =
            Endpoint::new(HttpMethod::Get, "/users/@me/channels").compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<ChannelResponse>>(&ep, None)
            .await
    }

    pub async fn create_dm(&self, body: &CreatePrivateChannelRequest) -> Result<ChannelResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/channels")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<CreatePrivateChannelRequest, ChannelResponse>(&ep, Some(body))
            .await
    }

    pub async fn pin_channel(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Put, "/users/@me/channels/{channel_id}/pin").compile(
            &QueryValues::new(),
            &[("channel_id", &channel_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn unpin_channel(&self, channel_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/channels/{channel_id}/pin")
            .compile(&QueryValues::new(), &[("channel_id", &channel_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_connections(&self) -> Result<Vec<ConnectionResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/connections")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<ConnectionResponse>>(&ep, None)
            .await
    }

    pub async fn create_connection(
        &self,
        body: &CreateConnectionRequest,
    ) -> Result<ConnectionVerificationResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<CreateConnectionRequest, ConnectionVerificationResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn authorize_bluesky(
        &self,
        body: &BlueskyAuthorizeRequest,
    ) -> Result<BlueskyAuthorizeResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections/bluesky/authorize")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<BlueskyAuthorizeRequest, BlueskyAuthorizeResponse>(&ep, Some(body))
            .await
    }

    pub async fn reorder_connections(&self, body: &ReorderConnectionsRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/connections/reorder")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn verify_connection(
        &self,
        body: &VerifyAndCreateConnectionRequest,
    ) -> Result<ConnectionResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/connections/verify")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<VerifyAndCreateConnectionRequest, ConnectionResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_connection(
        &self,
        connection_type: &str,
        connection_id: &str,
        body: &UpdateConnectionRequest,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/users/@me/connections/{type}/{connection_id}",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_connection(
        &self,
        connection_type: &str,
        connection_id: &str,
    ) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/users/@me/connections/{type}/{connection_id}",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn verify_connection_by_id(
        &self,
        connection_type: &str,
        connection_id: &str,
    ) -> Result<ConnectionResponse> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/connections/{type}/{connection_id}/verify",
        )
        .compile(
            &QueryValues::new(),
            &[("type", connection_type), ("connection_id", connection_id)],
        )?;
        self.http
            .request_json::<(), ConnectionResponse>(&ep, None)
            .await
    }

    pub async fn list_mentions(
        &self,
        query: &ListMentionsQuery,
    ) -> Result<Vec<Value>> {
        let mut q = QueryValues::new();
        q.insert_opt("limit", query.limit);
        q.insert_opt("roles", query.roles);
        q.insert_opt("everyone", query.everyone);
        q.insert_opt("guilds", query.guilds);
        q.insert_opt("before", query.before);

        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/mentions").compile(&q, &[])?;
        self.http.request_json::<(), Vec<Value>>(&ep, None).await
    }

    pub async fn delete_mention(&self, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/mentions/{message_id}")
            .compile(&QueryValues::new(), &[("message_id", &message_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_relationships(&self) -> Result<Vec<RelationshipResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/relationships")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<RelationshipResponse>>(&ep, None)
            .await
    }

    pub async fn create_relationship_by_tag(
        &self,
        body: &FriendRequestByTagRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/relationships")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<FriendRequestByTagRequest, RelationshipResponse>(&ep, Some(body))
            .await
    }

    pub async fn create_relationship(&self, user_id: Snowflake) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<(), RelationshipResponse>(&ep, None)
            .await
    }

    pub async fn put_relationship(
        &self,
        user_id: Snowflake,
        body: &RelationshipTypePutRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Put, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<RelationshipTypePutRequest, RelationshipResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_relationship(&self, user_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn update_relationship_nickname(
        &self,
        user_id: Snowflake,
        body: &RelationshipNicknameUpdateRequest,
    ) -> Result<RelationshipResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/relationships/{user_id}")
            .compile(&QueryValues::new(), &[("user_id", &user_id.to_string())])?;
        self.http
            .request_json::<RelationshipNicknameUpdateRequest, RelationshipResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn get_settings(&self) -> Result<UserSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/settings")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), UserSettingsResponse>(&ep, None)
            .await
    }

    pub async fn update_settings(
        &self,
        body: &UserSettingsUpdateRequest,
    ) -> Result<UserSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/settings")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<UserSettingsUpdateRequest, UserSettingsResponse>(&ep, Some(body))
            .await
    }

    pub async fn list_my_guilds(&self) -> Result<Vec<UserGuildResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/guilds")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<UserGuildResponse>>(&ep, None)
            .await
    }

    pub async fn leave_guild(&self, guild_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/guilds/{guild.id}")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn update_my_guild_settings(
        &self,
        body: &UserGuildSettingsUpdateRequest,
    ) -> Result<UserGuildSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/guilds/@me/settings")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<UserGuildSettingsUpdateRequest, UserGuildSettingsResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn update_guild_settings(
        &self,
        guild_id: Snowflake,
        body: &UserGuildSettingsUpdateRequest,
    ) -> Result<UserGuildSettingsResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/guilds/{guild.id}/settings")
            .compile(&QueryValues::new(), &[("guild.id", &guild_id.to_string())])?;
        self.http
            .request_json::<UserGuildSettingsUpdateRequest, UserGuildSettingsResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn list_notes(&self) -> Result<Vec<UserNoteResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/notes")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<UserNoteResponse>>(&ep, None)
            .await
    }

    pub async fn get_note(&self, target_id: Snowflake) -> Result<UserNoteResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/notes/{target.id}")
            .compile(&QueryValues::new(), &[("target.id", &target_id.to_string())])?;
        self.http
            .request_json::<(), UserNoteResponse>(&ep, None)
            .await
    }

    pub async fn put_note(
        &self,
        target_id: Snowflake,
        body: &UserNoteUpdateRequest,
    ) -> Result<UserNoteResponse> {
        let ep = Endpoint::new(HttpMethod::Put, "/users/@me/notes/{target.id}")
            .compile(&QueryValues::new(), &[("target.id", &target_id.to_string())])?;
        self.http
            .request_json::<UserNoteUpdateRequest, UserNoteResponse>(&ep, Some(body))
            .await
    }

    pub async fn list_saved_messages(&self) -> Result<Vec<SavedMessageResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/saved-messages")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<SavedMessageResponse>>(&ep, None)
            .await
    }

    pub async fn create_saved_message(
        &self,
        body: &SavedMessageCreateRequest,
    ) -> Result<SavedMessageResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/saved-messages")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<SavedMessageCreateRequest, SavedMessageResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_saved_message(&self, message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/saved-messages/{message.id}")
            .compile(&QueryValues::new(), &[("message.id", &message_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_scheduled_messages(&self) -> Result<Vec<ScheduledMessageResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/scheduled-messages")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<ScheduledMessageResponse>>(&ep, None)
            .await
    }

    pub async fn get_scheduled_message(
        &self,
        scheduled_message_id: Snowflake,
    ) -> Result<ScheduledMessageResponse> {
        let ep = Endpoint::new(
            HttpMethod::Get,
            "/users/@me/scheduled-messages/{scheduled_message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[("scheduled_message.id", &scheduled_message_id.to_string())],
        )?;
        self.http
            .request_json::<(), ScheduledMessageResponse>(&ep, None)
            .await
    }

    pub async fn update_scheduled_message(
        &self,
        scheduled_message_id: Snowflake,
        body: &ScheduledMessageUpdateRequest,
    ) -> Result<ScheduledMessageResponse> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/users/@me/scheduled-messages/{scheduled_message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[("scheduled_message.id", &scheduled_message_id.to_string())],
        )?;
        self.http
            .request_json::<ScheduledMessageUpdateRequest, ScheduledMessageResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn delete_scheduled_message(&self, scheduled_message_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/users/@me/scheduled-messages/{scheduled_message.id}",
        )
        .compile(
            &QueryValues::new(),
            &[("scheduled_message.id", &scheduled_message_id.to_string())],
        )?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_push_subscriptions(&self) -> Result<PushSubscriptionsListResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/push/subscriptions")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), PushSubscriptionsListResponse>(&ep, None)
            .await
    }

    pub async fn subscribe_push(
        &self,
        body: &PushSubscribeRequest,
    ) -> Result<PushSubscriptionResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/push/subscribe")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PushSubscribeRequest, PushSubscriptionResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_push_subscription(&self, subscription_id: &str) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/users/@me/push/subscriptions/{subscription.id}",
        )
        .compile(&QueryValues::new(), &[("subscription.id", subscription_id)])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn list_mfa_webauthn_credentials(
        &self,
    ) -> Result<Vec<MfaWebAuthnCredentialResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/mfa/webauthn/credentials")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), Vec<MfaWebAuthnCredentialResponse>>(&ep, None)
            .await
    }

    pub async fn create_mfa_webauthn_credential(
        &self,
        body: &MfaWebAuthnCredentialCreateRequest,
    ) -> Result<MfaWebAuthnCredentialResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/webauthn/credentials")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<MfaWebAuthnCredentialCreateRequest, MfaWebAuthnCredentialResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn get_mfa_webauthn_registration_options(
        &self,
        body: &MfaWebAuthnRegistrationOptionsRequest,
    ) -> Result<Value> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/mfa/webauthn/credentials/registration-options",
        )
        .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<MfaWebAuthnRegistrationOptionsRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn delete_mfa_webauthn_credential(&self, credential_id: &str) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Delete,
            "/users/@me/mfa/webauthn/credentials/{credential.id}",
        )
        .compile(&QueryValues::new(), &[("credential.id", credential_id)])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn update_mfa_webauthn_credential(
        &self,
        credential_id: &str,
        body: &MfaWebAuthnCredentialUpdateRequest,
    ) -> Result<MfaWebAuthnCredentialResponse> {
        let ep = Endpoint::new(
            HttpMethod::Patch,
            "/users/@me/mfa/webauthn/credentials/{credential.id}",
        )
        .compile(&QueryValues::new(), &[("credential.id", credential_id)])?;
        self.http
            .request_json::<MfaWebAuthnCredentialUpdateRequest, MfaWebAuthnCredentialResponse>(
                &ep,
                Some(body),
            )
            .await
    }

    pub async fn generate_mfa_backup_codes(
        &self,
        body: &MfaActionRequest,
    ) -> Result<MfaBackupCodesResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/backup-codes")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<MfaActionRequest, MfaBackupCodesResponse>(&ep, Some(body))
            .await
    }

    pub async fn enable_mfa_totp(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/totp/enable")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn disable_mfa_totp(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/totp/disable")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn enable_mfa_sms(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/sms/enable")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn disable_mfa_sms(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/mfa/sms/disable")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn add_phone(&self, body: &PhoneActionRequest) -> Result<UserPrivateResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/phone")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PhoneActionRequest, UserPrivateResponse>(&ep, Some(body))
            .await
    }

    pub async fn send_phone_verification(&self, body: &PhoneActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/phone/send-verification")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn verify_phone(&self, body: &PhoneActionRequest) -> Result<UserPrivateResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/phone/verify")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<PhoneActionRequest, UserPrivateResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_phone(&self, body: &PhoneActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/phone")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_start(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/start")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_verify_original(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/verify-original")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_request_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/request-new")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_resend_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/resend-new")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_resend_original(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/resend-original")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_verify_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/email-change/verify-new")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_bounced_request_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/email-change/bounced/request-new",
        )
        .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_bounced_resend_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/email-change/bounced/resend-new",
        )
        .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn email_change_bounced_verify_new(&self, body: &EmailChangeRequest) -> Result<()> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/email-change/bounced/verify-new",
        )
        .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn password_change_start(&self, body: &PasswordChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/password-change/start")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn password_change_verify(&self, body: &PasswordChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/password-change/verify")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn password_change_resend(&self, body: &PasswordChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/password-change/resend")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn password_change_complete(&self, body: &PasswordChangeRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/password-change/complete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_account(&self, body: &AccountDeleteRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn disable_account(&self, body: &AccountDisableRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/disable")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn delete_authorized_ips(&self) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/authorized-ips")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_sudo_mfa_methods(&self) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/sudo/mfa-methods")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn send_sudo_mfa_sms(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/sudo/mfa/sms/send")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn get_sudo_webauthn_options(&self, body: &MfaActionRequest) -> Result<Value> {
        let ep = Endpoint::new(
            HttpMethod::Post,
            "/users/@me/sudo/webauthn/authentication-options",
        )
        .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<MfaActionRequest, Value>(&ep, Some(body))
            .await
    }

    pub async fn get_harvest_latest(&self) -> Result<HarvestResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/harvest/latest")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<(), HarvestResponse>(&ep, None)
            .await
    }

    pub async fn get_harvest(&self, harvest_id: Snowflake) -> Result<HarvestResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/harvest/{harvest.id}")
            .compile(&QueryValues::new(), &[("harvest.id", &harvest_id.to_string())])?;
        self.http
            .request_json::<(), HarvestResponse>(&ep, None)
            .await
    }

    pub async fn get_harvest_download(&self, harvest_id: Snowflake) -> Result<Value> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/harvest/{harvest.id}/download")
            .compile(&QueryValues::new(), &[("harvest.id", &harvest_id.to_string())])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn create_harvest(&self, body: &HarvestCreateRequest) -> Result<HarvestResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/harvest")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<HarvestCreateRequest, HarvestResponse>(&ep, Some(body))
            .await
    }

    pub async fn list_memes(&self) -> Result<Vec<MemeResponse>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/memes")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Vec<MemeResponse>>(&ep, None).await
    }

    pub async fn get_meme(&self, meme_id: Snowflake) -> Result<MemeResponse> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/memes/{meme.id}")
            .compile(&QueryValues::new(), &[("meme.id", &meme_id.to_string())])?;
        self.http.request_json::<(), MemeResponse>(&ep, None).await
    }

    pub async fn create_meme(&self, body: &MemeCreateRequest) -> Result<MemeResponse> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/memes")
            .compile(&QueryValues::new(), &[])?;
        self.http
            .request_json::<MemeCreateRequest, MemeResponse>(&ep, Some(body))
            .await
    }

    pub async fn update_meme(&self, meme_id: Snowflake, body: &MemeUpdateRequest) -> Result<MemeResponse> {
        let ep = Endpoint::new(HttpMethod::Patch, "/users/@me/memes/{meme.id}")
            .compile(&QueryValues::new(), &[("meme.id", &meme_id.to_string())])?;
        self.http
            .request_json::<MemeUpdateRequest, MemeResponse>(&ep, Some(body))
            .await
    }

    pub async fn delete_meme(&self, meme_id: Snowflake) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/memes/{meme.id}")
            .compile(&QueryValues::new(), &[("meme.id", &meme_id.to_string())])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn get_my_applications(&self) -> Result<Vec<Value>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/applications")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Vec<Value>>(&ep, None).await
    }

    pub async fn get_my_gifts(&self) -> Result<Vec<Value>> {
        let ep = Endpoint::new(HttpMethod::Get, "/users/@me/gifts")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_json::<(), Vec<Value>>(&ep, None).await
    }

    pub async fn start_delete_my_messages(&self, body: &BulkMessageDeleteRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/messages/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn cancel_delete_my_messages(&self) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Delete, "/users/@me/messages/delete")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit::<()>(&ep, None).await
    }

    pub async fn test_delete_my_messages(&self, body: &BulkMessageDeleteRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/messages/delete/test")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn preload_channel_messages(&self, body: &PreloadMessagesRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/channels/messages/preload")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn preload_messages(&self, body: &PreloadMessagesRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/preload-messages")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn apply_themes(&self, body: &ThemesRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/themes")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn reset_premium(&self, body: &MfaActionRequest) -> Result<()> {
        let ep = Endpoint::new(HttpMethod::Post, "/users/@me/premium/reset")
            .compile(&QueryValues::new(), &[])?;
        self.http.request_unit(&ep, Some(body)).await
    }

    pub async fn check_tag(&self, query: &CheckTagQuery) -> Result<Value> {
        let mut q = QueryValues::new();
        q.insert("tag", &query.tag);
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/users/check-tag").compile(&q, &[])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_user(&self, user_id: Snowflake) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/users/{user.id}")
            .compile(&QueryValues::new(), &[("user.id", &user_id.to_string())])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }

    pub async fn get_user_profile(&self, target_id: Snowflake) -> Result<Value> {
        let ep = Endpoint::new_no_bot_auth(HttpMethod::Get, "/users/{target.id}/profile")
            .compile(&QueryValues::new(), &[("target.id", &target_id.to_string())])?;
        self.http.request_json::<(), Value>(&ep, None).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_bounced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfileUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_masked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_timestamp_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_badge_sequence_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_enabled_override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_dismissed_premium_onboarding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_unread_gift_inventory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_mobile_client: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrivateChannelRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<Snowflake>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub name: String,
    pub verified: bool,
    pub visibility_flags: i32,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionVerificationResponse {
    pub token: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub id: String,
    pub instructions: String,
    pub initiation_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "type")]
    pub connection_type: String,
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueskyAuthorizeRequest {
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueskyAuthorizeResponse {
    pub authorize_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderConnectionsRequest {
    pub connection_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAndCreateConnectionRequest {
    pub initiation_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConnectionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_flags: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMentionsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guilds: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipResponse {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: i32,
    pub user: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendRequestByTagRequest {
    pub username: String,
    pub discriminator: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationshipTypePutRequest {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNicknameUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsUpdateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildResponse {
    pub id: Snowflake,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserGuildSettingsUpdateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsResponse {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNoteResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNoteUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedMessageResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessageCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMessageResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduledMessageUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscriptionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subscription_id", alias = "id")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscriptionsListResponse {
    #[serde(default)]
    pub subscriptions: Vec<PushSubscriptionResponse>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushSubscribeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p256dh: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaWebAuthnCredentialResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaWebAuthnCredentialCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaWebAuthnRegistrationOptionsRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaWebAuthnCredentialUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaActionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaBackupCodesResponse {
    #[serde(default)]
    pub codes: Vec<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneActionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailChangeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountDeleteRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_response: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountDisableRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarvestCreateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemeCreateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemeUpdateRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkMessageDeleteRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreloadMessagesRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemesRequest {
    #[serde(flatten)]
    pub raw: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTagQuery {
    pub tag: String,
}
