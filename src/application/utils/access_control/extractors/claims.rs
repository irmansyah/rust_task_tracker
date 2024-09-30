use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use derive_more::Display;
use jsonwebtoken::jwk::AlgorithmParameters;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, str::FromStr};

// Define roles with their associated permissions
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    SuperAdmin,
    Admin,
    Author,
    Customer,
}

impl Default for Role {
    fn default() -> Self {
        Role::Customer
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::SuperAdmin => write!(f, "super_admin"),
            Role::Admin => write!(f, "admin"),
            Role::Author => write!(f, "author"),
            Role::Customer => write!(f, "customer"),
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "super_admin" => Ok(Role::SuperAdmin),
            "admin" => Ok(Role::Admin),
            "author" => Ok(Role::Author),
            "customer" => Ok(Role::Customer),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

// Permissions like read, write, delete actions for resources
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Permission {
    Read(String),   // e.g., "read:admin-tasks"
    Write(String),  // e.g., "write:admin-tasks"
    Delete(String), // e.g., "delete:admin-tasks"
}

// JWT Claims structure to include both role and permissions
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,                              // Subject (user ID)
    pub role: Role,                               // User role
    pub permissions: Option<HashSet<Permission>>, // Optional specific permissions
    pub exp: usize,                               // Expiry timestamp
}

#[derive(Debug, Display)]
pub enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
}

impl Claims {
    // Check if the user has the required role and permissions
    pub fn validate_roles(&self, allowed_roles: &[Role]) -> bool {
        allowed_roles.contains(&self.role)
    }

    // Check if the user has the required permissions
    pub fn validate_permissions(&self, required_permissions: &HashSet<Permission>) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| permissions.is_superset(required_permissions))
    }
}
