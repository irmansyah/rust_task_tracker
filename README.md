# Notion Task Management Clone

A Rust project inspired by Notion's task management system, built with a focus on:

- **Rust** for the backend logic
- **PostgreSQL** as the database
- **Nginx** for reverse proxy and load balancing
- **Clean Architecture** to ensure maintainability and scalability
- **JWT Authentication** for secure access
- **Access Control** to manage roles and permissions
- **Refresh Token** mechanism for token renewal and session management
## Getting Started

To run the project using Docker, follow these steps:

1. Make sure you have Docker and Docker Compose installed.
2. Clone the repository.
3. Run the following command to start the application:

```bash
    cp .env.example .env
    docker-compose -f docker-compose.dev.yaml up -d --build
```

## Acknowledgements

I would like to thank the following repositories for providing inspiration and guidance during the development of this project:

- [MSC29/clean-architecture-rust](https://github.com/MSC29/clean-architecture-rust) - Inspiration for implementing clean architecture and more.
- [api_actix-web_rust_hello-world](https://github.com/auth0-developer-hub/api_actix-web_rust_hello-world.git) - Inspiration for access control mechanisms.

Your contributions and ideas have been invaluable. 🙏
