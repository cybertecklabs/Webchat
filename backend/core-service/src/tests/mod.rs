#[cfg(test)]
mod server_tests {
    use crate::models::*;
    use mongodb::bson::oid::ObjectId;

    #[test]
    fn test_server_creation() {
        let server = Server {
            id: Some(ObjectId::new()),
            name: "Test Server".to_string(),
            owner_id: ObjectId::new(),
            invite_code: "ABC123".to_string(),
            created_at: chrono::Utc::now(),
        };
        
        assert_eq!(server.name, "Test Server");
        assert_eq!(server.invite_code.len(), 6);
    }
}

#[cfg(test)]
mod cache_tests {
    // Note: This requires a running Redis for testing, or mocking the cache manager.
    // For now, these are unit-level structure tests if any, or integration-ready logic.
}
