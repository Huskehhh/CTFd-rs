use async_trait::async_trait;
use ctfdb::DiscordNameProvider;
use serenity::http::Http;

pub struct AsyncDiscordNameProvider<'a> {
    pub http: &'a Http,
    pub guild_id: u64,
}

#[async_trait]
impl DiscordNameProvider for AsyncDiscordNameProvider<'_> {
    async fn name_for_id(&self, id: i64) -> Option<String> {
        if let Ok(member) = &self.http.get_member(self.guild_id, id as u64).await {
            let name = member.nick.as_ref().unwrap_or(&member.user.name);
            return Some(name.clone());
        }

        None
    }
}
