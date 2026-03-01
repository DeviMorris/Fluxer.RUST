use dashmap::DashMap;

use fluxer_types::user::ApiUser;

use crate::structures::user::User;

/// Manages cached users.
///
/// # Examples
/// ```rust,ignore
/// let user = client.users.get("123456789");
/// let profile = client.users.fetch_with_profile("123456789").await?;
/// ```
pub struct UsersManager<'a> {
    cache: &'a DashMap<String, User>,
    rest: &'a fluxer_rest::Rest,
}

impl<'a> UsersManager<'a> {
    pub fn new(cache: &'a DashMap<String, User>, rest: &'a fluxer_rest::Rest) -> Self {
        Self { cache, rest }
    }

    /// Get a cached user by ID.
    pub fn get(&self, id: &str) -> Option<User> {
        self.cache.get(id).map(|r| r.clone())
    }

    /// Fetch a user from the API.
    pub async fn fetch(&self, id: &str) -> crate::Result<User> {
        let data: ApiUser = self
            .rest
            .get(&fluxer_types::Routes::user(id))
            .await?;
        let user = User::from_api(&data);
        self.cache.insert(user.id.clone(), user.clone());
        Ok(user)
    }

    /// Resolve a user — from cache or API.
    pub async fn resolve(&self, id: &str) -> crate::Result<User> {
        if let Some(u) = self.get(id) {
            return Ok(u);
        }
        self.fetch(id).await
    }

    /// Fetch user with profile data (banner etc).
    pub async fn fetch_with_profile(&self, id: &str) -> crate::Result<fluxer_types::user::ApiProfileResponse> {
        let data: fluxer_types::user::ApiProfileResponse = self
            .rest
            .get(&fluxer_types::Routes::user_profile(id, None))
            .await?;
        Ok(data)
    }
}
