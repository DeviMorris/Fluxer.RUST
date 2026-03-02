use fluxer_types::Snowflake;

#[derive(Debug, Clone)]
pub struct MessageReaction {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: String,
    pub emoji_animated: bool,
}

impl MessageReaction {
    pub fn from_gateway(data: &fluxer_types::gateway::GatewayReactionAddData) -> Self {
        Self {
            message_id: data.message_id.clone(),
            channel_id: data.channel_id.clone(),
            guild_id: data.guild_id.clone(),
            user_id: data.user_id.clone(),
            emoji_id: data.emoji.id.clone(),
            emoji_name: data.emoji.name.clone(),
            emoji_animated: data.emoji.animated.unwrap_or(false),
        }
    }

    pub fn emoji_identifier(&self) -> String {
        match &self.emoji_id {
            Some(id) => format!("{}:{}", self.emoji_name, id),
            None => self.emoji_name.clone(),
        }
    }
}
