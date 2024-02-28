use crate::errors::Errors;
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs};

#[derive(Debug)]
pub struct AsyncClient(Client);

impl AsyncClient {
    pub fn open(redis_uri: String) -> Result<AsyncClient, Errors> {
        Ok(AsyncClient(Client::open(redis_uri)?))
    }

    pub async fn set_ex<K: ToRedisArgs + Send + Sync, V: ToRedisArgs + Send + Sync>(
        &self,
        key: K,
        value: V,
        seconds: u64,
    ) -> Result<(), Errors> {
        Ok(self.0.get_async_connection().await?.set_ex(key, value, seconds).await?)
    }

    pub async fn get<T: FromRedisValue, K: ToRedisArgs + Send + Sync>(&self, key: K) -> Result<T, Errors> {
        let value = self.0.get_async_connection().await?.get(key).await?;
        Ok(value)
    }
}

impl From<Client> for AsyncClient {
    fn from(value: Client) -> Self {
        AsyncClient(value)
    }
}
