use redis::{Client, Connection, Commands, RedisError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    fn get_connection(&self) -> Result<Connection, RedisError> {
        self.client.get_connection()
    }

    pub async fn set<K, V>(&self, key: K, value: &V) -> Result<(), RedisError>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let mut conn = self.get_connection()?;
        let serialized = serde_json::to_string(value)
            .map_err(|e| RedisError::from((
                redis::ErrorKind::InvalidArgument,
                "Serialization failed",
                e.to_string(),
            )))?;
        
        conn.set(key.as_ref(), serialized)
    }

    pub async fn get<K, V>(&self, key: K) -> Result<Option<V>, RedisError>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection()?;
        let result: Option<String> = conn.get(key.as_ref())?;
        
        if let Some(serialized) = result {
            let deserialized = serde_json::from_str(&serialized)
                .map_err(|e| RedisError::from((
                    redis::ErrorKind::InvalidArgument,
                    "Deserialization failed",
                    e.to_string(),
                )))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    pub async fn set_with_ttl<K, V>(&self, key: K, value: &V, ttl_seconds: u64) -> Result<(), RedisError>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let mut conn = self.get_connection()?;
        let serialized = serde_json::to_string(value)
            .map_err(|e| RedisError::from((
                redis::ErrorKind::InvalidArgument,
                "Serialization failed",
                e.to_string(),
            )))?;
        
        let mut pipe = redis::pipe();
        pipe.set(key.as_ref(), serialized)
            .expire(key.as_ref(), ttl_seconds as usize);
        
        pipe.execute(&mut conn)?;
        Ok(())
    }

    pub async fn delete<K>(&self, key: K) -> Result<bool, RedisError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.get_connection()?;
        let result: i32 = conn.del(key.as_ref())?;
        Ok(result > 0)
    }

    pub async fn clear_pattern(&self, pattern: &str) -> Result<u64, RedisError> {
        let mut conn = self.get_connection()?;
        let keys: Vec<String> = conn.keys(pattern)?;
        
        if keys.is_empty() {
            return Ok(0);
        }
        
        let result: i32 = conn.del(&keys)?;
        Ok(result as u64)
    }

    pub async fn exists<K>(&self, key: K) -> Result<bool, RedisError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.get_connection()?;
        let result: i32 = conn.exists(key.as_ref())?;
        Ok(result > 0)
    }

    pub async fn increment<K>(&self, key: K, amount: i64) -> Result<i64, RedisError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.get_connection()?;
        conn.incr(key.as_ref(), amount)
    }

    pub async fn get_ttl<K>(&self, key: K) -> Result<Option<u64>, RedisError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.get_connection()?;
        let result: i32 = conn.ttl(key.as_ref())?;
        
        if result > 0 {
            Ok(Some(result as u64))
        } else if result == -1 {
            Ok(None) // Sem TTL
        } else {
            Ok(Some(0)) // Expirou
        }
    }

    pub async fn set_hash<K, F, V>(&self, key: K, field: F, value: &V) -> Result<(), RedisError>
    where
        K: AsRef<str>,
        F: AsRef<str>,
        V: Serialize,
    {
        let mut conn = self.get_connection()?;
        let serialized = serde_json::to_string(value)
            .map_err(|e| RedisError::from((
                redis::ErrorKind::InvalidArgument,
                "Serialization failed",
                e.to_string(),
            )))?;
        
        conn.hset(key.as_ref(), field.as_ref(), serialized)
    }

    pub async fn get_hash<K, F, V>(&self, key: K, field: F) -> Result<Option<V>, RedisError>
    where
        K: AsRef<str>,
        F: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection()?;
        let result: Option<String> = conn.hget(key.as_ref(), field.as_ref())?;
        
        if let Some(serialized) = result {
            let deserialized = serde_json::from_str(&serialized)
                .map_err(|e| RedisError::from((
                    redis::ErrorKind::InvalidArgument,
                    "Deserialization failed",
                    e.to_string(),
                )))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_hash<K, V>(&self, key: K) -> Result<HashMap<String, V>, RedisError>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection()?;
        let result: HashMap<String, String> = conn.hgetall(key.as_ref())?;
        
        let mut deserialized = HashMap::new();
        for (field, serialized) in result {
            let value: V = serde_json::from_str(&serialized)
                .map_err(|e| RedisError::from((
                    redis::ErrorKind::InvalidArgument,
                    "Deserialization failed",
                    e.to_string(),
                )))?;
            deserialized.insert(field, value);
        }
        
        Ok(deserialized)
    }

    pub async fn delete_hash<K, F>(&self, key: K, field: F) -> Result<bool, RedisError>
    where
        K: AsRef<str>,
        F: AsRef<str>,
    {
        let mut conn = self.get_connection()?;
        let result: i32 = conn.hdel(key.as_ref(), field.as_ref())?;
        Ok(result > 0)
    }

    pub async fn ping(&self) -> Result<String, RedisError> {
        let mut conn = self.get_connection()?;
        conn.ping()
    }

    pub async fn flush_all(&self) -> Result<(), RedisError> {
        let mut conn = self.get_connection()?;
        conn.flushall()
    }
}
