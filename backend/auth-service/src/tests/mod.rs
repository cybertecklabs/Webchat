#[cfg(test)]
mod auth_tests {
    use crate::auth::{hash_password, verify_password, create_jwt};

    #[test]
    fn test_password_hashing() {
        let password = "secure_password_123";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_password_hash_uniqueness() {
        let password = "same_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Hashes should be different due to salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_jwt_creation() {
        // Mock JWT_SECRET for tests if not set
        if std::env::var("JWT_SECRET").is_err() {
            std::env::set_var("JWT_SECRET", "test_secret_key");
        }
        
        let user_id = "507f1f77bcf86cd799439011";
        let token = create_jwt(user_id);
        
        assert!(!token.is_empty());
        assert!(token.contains('.'));  // JWT format
    }
}

#[cfg(test)]
mod validation_tests {
    use crate::models::RegisterRequest;
    use validator::Validate;

    #[test]
    fn test_valid_registration() {
        let req = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let req = RegisterRequest {
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_short_password() {
        let req = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_short_username() {
        let req = RegisterRequest {
            username: "ab".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(req.validate().is_err());
    }
}
