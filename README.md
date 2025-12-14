# RSS Feed Aggregator API

A production-grade RSS feed aggregator and reader API built with Rust, designed to teach modern async Rust practices through progressive complexity.

## Project Overview

This project is a feature-rich RSS/Atom feed aggregator that provides a REST API for subscribing to feeds, fetching articles, and managing reading lists. It's designed as a learning project that progresses from basic to advanced Rust concepts while building something genuinely useful.

**Why this project?**
- Teaches all major async Rust crates in practical contexts
- Builds production-ready skills (error handling, testing, database design, API design)
- Results in a deployable open-source tool
- Covers real-world challenges (rate limiting, caching, concurrent updates, data persistence)

## Core Technologies

- **tokio** - Async runtime for concurrent feed fetching
- **axum** - Modern, ergonomic web framework
- **reqwest** - HTTP client for fetching feeds
- **serde** - Serialization/deserialization for JSON APIs and data models
- **sqlx** - Async database access (PostgreSQL)
- **tower** - Middleware for rate limiting, tracing, and timeouts

## Architecture

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ HTTP/JSON
┌──────▼──────────────────────────┐
│   Axum REST API Layer           │
│  (Routes, Auth, Validation)     │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│   Business Logic Layer          │
│  (Feed Manager, Article Parser) │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│   Data Access Layer (SQLx)      │
└──────┬──────────────────────────┘
       │
┌──────▼──────────────────────────┐
│   PostgreSQL Database           │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│  Background Worker (Tokio)      │
│  - Feed Fetcher (reqwest)       │
│  - RSS/Atom Parser (quick-xml)  │
│  - Article Processor            │
└─────────────────────────────────┘
```

---

## Phase 1: Foundation (Basics)

**Goal**: Set up project structure, basic HTTP server, simple RSS fetching

### Learning Objectives
- Project setup with Cargo workspaces
- Basic axum server with routing
- Simple async operations with tokio
- HTTP requests with reqwest
- Basic serde JSON serialization

### Features to Implement
1. **Project Setup**
   ```toml
   [workspace]
   members = ["models", "feed-fetcher"]
   ```

2. **Basic REST API** (axum)
   - `GET /health` - Health check endpoint
   - `GET /api/feeds` - List all subscribed feeds
   - `POST /api/feeds` - Subscribe to a new feed
   - `DELETE /api/feeds/:id` - Unsubscribe from feed

3. **Simple Feed Fetcher** (reqwest + tokio)
   - Fetch RSS/Atom feed from URL
   - Parse XML to extract title, link, description
   - Return parsed data as JSON

4. **Data Models** (serde)
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   struct Feed {
       id: Uuid,
       url: String,
       title: String,
       description: Option<String>,
   }

   #[derive(Debug, Serialize, Deserialize)]
   struct Article {
       id: Uuid,
       feed_id: Uuid,
       title: String,
       link: String,
       published: DateTime<Utc>,
       content: String,
   }
   ```

### Key Learnings
- `#[tokio::main]` for async main
- `axum::Router` and handler functions
- `reqwest::Client` for HTTP requests
- `serde` derive macros
- Basic error handling with `Result<T, E>`

### Deliverables
- Working HTTP server on `localhost:3000`
- Can add feed URL and fetch its content
- Proper project structure with modules
- Basic unit tests

---

## Phase 2: Database & Persistence (Intermediate)

**Goal**: Add PostgreSQL database, persist feeds and articles, implement CRUD operations

### Learning Objectives
- Async database operations with sqlx
- Connection pooling
- Database migrations
- Transaction handling
- More sophisticated error handling

### Features to Implement
1. **Database Setup** (sqlx)
   - PostgreSQL connection pool
   - Migrations for feeds and articles tables
   - Repository pattern for data access

2. **Enhanced API Endpoints**
   - `GET /api/articles` - List articles (with pagination)
   - `GET /api/articles/:id` - Get single article
   - `PUT /api/articles/:id/read` - Mark article as read
   - `GET /api/feeds/:id/articles` - Get articles for specific feed

3. **Background Feed Updater**
   - Tokio task that runs every 15 minutes
   - Fetches all subscribed feeds concurrently
   - Stores new articles in database
   - Handles duplicate detection

4. **Query Parameters & Filtering**
   ```rust
   #[derive(Deserialize)]
   struct ArticleQuery {
       feed_id: Option<Uuid>,
       unread_only: Option<bool>,
       limit: Option<i64>,
       offset: Option<i64>,
   }
   ```

### Key Learnings
- `sqlx::PgPool` for connection pooling
- `sqlx::query!` and `query_as!` macros
- Database transactions
- `tokio::spawn` for background tasks
- `tokio::time::interval` for periodic tasks
- Custom error types implementing `std::error::Error`
- `axum::extract::Query` for query parameters

### Deliverables
- Fully persistent application
- Background worker updating feeds
- Pagination support
- Database-backed CRUD operations

---

## Phase 3: Advanced Features (Intermediate-Advanced)

**Goal**: Add authentication, caching, rate limiting, advanced parsing

### Learning Objectives
- JWT authentication
- Tower middleware
- Redis caching
- Concurrent request handling
- Advanced serde techniques

### Features to Implement
1. **Authentication & Authorization**
   - User registration and login
   - JWT token generation and validation
   - Protected routes (axum middleware)
   - User-specific feed subscriptions

2. **Caching Layer** (redis)
   - Cache parsed feed data (15-minute TTL)
   - Cache frequently accessed articles
   - Implement cache-aside pattern

3. **Rate Limiting** (tower-governor)
   - Per-IP rate limiting
   - Per-user rate limiting for authenticated endpoints
   - Custom rate limit responses

4. **Advanced Feed Parsing**
   - Support both RSS 2.0 and Atom 1.0
   - Handle various encodings
   - Extract media enclosures (podcasts, images)
   - Full-text content extraction from partial feeds

5. **Search Functionality**
   - Full-text search across articles
   - PostgreSQL full-text search or Tantivy integration
   - `GET /api/search?q=rust+programming`

### Key Learnings
- `tower::ServiceBuilder` for middleware
- `axum::middleware::from_fn` for custom middleware
- JWT libraries (jsonwebtoken)
- Redis client (redis-rs)
- Advanced serde attributes (`#[serde(rename)]`, `#[serde(flatten)]`)
- Stream processing with `futures::Stream`

### Deliverables
- Multi-user support with authentication
- Cached responses for better performance
- Rate-limited API
- Comprehensive feed format support

---

## Phase 4: Production-Ready (Advanced)

**Goal**: Observability, error handling, testing, deployment readiness

### Learning Objectives
- Structured logging and tracing
- Comprehensive error handling
- Integration testing
- Graceful shutdown
- Configuration management
- Deployment considerations

### Features to Implement
1. **Observability**
   - Structured logging with `tracing`
   - Distributed tracing with trace IDs
   - Metrics with `prometheus` exporter
   - Health check with dependency status

2. **Robust Error Handling**
   - Custom error types with `thiserror`
   - Proper HTTP status codes
   - Error response formatting
   - Retry logic with exponential backoff

3. **Testing Suite**
   - Unit tests for business logic
   - Integration tests with `sqlx::test`
   - API endpoint tests with `axum-test`
   - Mock HTTP responses for feed fetching

4. **Configuration Management**
   - Environment-based config (dev, staging, prod)
   - `config` crate or `figment`
   - Secrets management (not hardcoded)

5. **Graceful Shutdown**
   - Handle SIGTERM/SIGINT
   - Drain in-flight requests
   - Close database connections cleanly

6. **API Documentation**
   - OpenAPI/Swagger spec generation
   - `utoipa` for automatic API docs
   - Serve docs at `/api/docs`

### Key Learnings
- `tracing` macros and subscribers
- `thiserror` for error types
- `#[cfg(test)]` and test organization
- `tokio::signal` for graceful shutdown
- `utoipa` for OpenAPI generation
- Docker and `docker-compose`

### Deliverables
- Production-grade error handling
- Comprehensive test coverage (>70%)
- Metrics and monitoring
- Docker deployment setup
- Complete API documentation

---

## Phase 5: Advanced Optimizations (Expert)

**Goal**: Performance optimization, advanced concurrency, scalability

### Learning Objectives
- Lock-free data structures
- Advanced tokio patterns
- Connection pooling strategies
- Lazy evaluation and streaming
- Performance profiling

### Features to Implement
1. **Performance Optimizations**
   - Connection reuse for feed fetching
   - Batch database operations
   - Lazy loading of article content
   - Response compression (gzip, brotli)

2. **Advanced Concurrency**
   - `tokio::select!` for timeout handling
   - `tokio::sync::Semaphore` for concurrent feed updates
   - Work-stealing scheduler optimization
   - Channel-based actor pattern for feed updates

3. **Streaming Responses**
   - Server-sent events (SSE) for real-time updates
   - Stream large paginated responses
   - WebSocket support for live notifications

4. **Scalability Features**
   - Horizontal scaling considerations
   - Database read replicas support
   - Distributed caching with Redis cluster
   - Message queue (e.g., NATS) for feed update jobs

5. **Advanced Features**
   - OPML import/export (feed list backup)
   - Feed autodiscovery from website URLs
   - Article recommendations using similarity
   - Webhook notifications for new articles

### Key Learnings
- `Arc` and `Mutex` vs `RwLock`
- Lock-free programming with atomic operations
- Benchmarking with `criterion`
- Profiling with `perf` and `flamegraph`
- `tokio::select!` and `tokio::join!`
- SSE with `axum::response::sse`

### Deliverables
- Highly optimized feed fetching (<100ms p99)
- Real-time notification system
- Scalable architecture (multiple server instances)
- Comprehensive benchmarks

---

## Project Structure

```
albatross/                  # Root is the main server package
├── Cargo.toml              # Workspace + albatross server package config
├── Cargo.lock              # Dependency lock file
├── README.md               # Project documentation
│
├── Dockerfile              # Production Docker image
├── docker-compose.yml      # Production Docker Compose
├── docker-compose.dev.yml  # Development Docker Compose
├── .dockerignore           # Docker build context exclusions
│
├── .env                    # Environment variables (not committed)
├── .env.example            # Example environment variables
├── .gitignore              # Git exclusions
│
├── prometheus.yml          # Prometheus configuration (optional)
│
├── src/                    # Main API server (albatross binary)
│   ├── main.rs
│   ├── routes/             # Axum route handlers
│   │   ├── mod.rs
│   │   ├── feeds.rs
│   │   ├── articles.rs
│   │   └── auth.rs
│   ├── middleware/         # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── rate_limit.rs
│   ├── handlers/           # Request handlers
│   │   ├── mod.rs
│   │   ├── feed_handler.rs
│   │   └── article_handler.rs
│   ├── config.rs           # Configuration
│   └── error.rs            # Error types
│
├── tests/                  # Integration tests for server
│   ├── integration_test.rs
│   └── common/
│
├── models/                 # Shared data models (library)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── feed.rs         # Feed data model
│       ├── article.rs      # Article data model
│       ├── user.rs         # User data model (Phase 3)
│       └── error.rs        # Shared error types
│
├── feed-fetcher/           # Feed fetching & parsing (library)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── fetcher.rs      # HTTP fetching with reqwest
│       ├── parser.rs       # RSS/Atom XML parsing
│       ├── worker.rs       # Background worker (tokio)
│       └── cache.rs        # Feed caching (Phase 3)
│
├── database/               # Database layer (library, optional Phase 2+)
│   ├── Cargo.toml
│   ├── migrations/         # SQLx SQL migrations
│   │   ├── 001_create_feeds.sql
│   │   ├── 002_create_articles.sql
│   │   └── 003_create_users.sql
│   └── src/
│       ├── lib.rs
│       ├── pool.rs         # Connection pool
│       └── repositories/   # Repository pattern
│           ├── mod.rs
│           ├── feed_repo.rs
│           ├── article_repo.rs
│           └── user_repo.rs
│
└── integration-tests/      # End-to-end tests (optional)
    ├── Cargo.toml
    └── tests/
        ├── api_tests.rs
        ├── feed_tests.rs
        └── common/
            └── setup.rs
```

### Key Files Explained

- **Dockerfile**: Multi-stage build for optimized production images
- **docker-compose.yml**: Orchestrates API, PostgreSQL, Redis services
- **docker-compose.dev.yml**: Development setup with hot-reload
- **.env**: Local environment variables (never commit)
- **.env.example**: Template for required environment variables
- **migrations/**: Database schema version control (SQLx)
- **repositories/**: Data access layer abstractions

---

## Getting Started

### Prerequisites

**Option 1: Using Docker (Recommended for Quick Start)**
```bash
# Install Docker and Docker Compose
# macOS: Download from https://www.docker.com/products/docker-desktop
# Linux: sudo apt-get install docker.io docker-compose
# Windows: Download from https://www.docker.com/products/docker-desktop

# Verify installation
docker --version
docker-compose --version
```

**Option 2: Local Installation**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
brew install postgresql  # macOS
sudo apt-get install postgresql  # Ubuntu/Debian

# Install Redis (Phase 3+)
brew install redis  # macOS
sudo apt-get install redis-server  # Ubuntu/Debian

# Install SQLx CLI for migrations
cargo install sqlx-cli --no-default-features --features postgres
```

### Quick Start with Docker

The fastest way to get the project running:

```bash
# 1. Clone or create the project structure
mkdir rss-aggregator && cd rss-aggregator

# 2. Create the workspace structure
cargo new --lib models
cargo new server
cargo new --lib feed-fetcher
cargo new --lib database

# 3. Create docker-compose.yml (see Deployment section for full config)
# Copy the docker-compose.yml from the Deployment section

# 4. Create .env file
cat > .env << EOF
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=rss_aggregator
REDIS_URL=redis://redis:6379
JWT_SECRET=dev-secret-key-change-in-production
RUST_LOG=info
PORT=3000
EOF

# 5. Start all services
docker-compose up -d

# 6. View logs
docker-compose logs -f api

# 7. Test the API
curl http://localhost:3000/health
```

### Phase 1 Quick Start (Local Development)

```bash
# 1. Create new project (root will be the server)
cargo new albatross
cd albatross

# 2. Update Cargo.toml to be a workspace
cat > Cargo.toml << EOF
[workspace]
members = ["models", "feed-fetcher"]
resolver = "2"

[package]
name = "albatross"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
EOF

# 3. Create library workspace members
cargo new --lib models
cargo new --lib feed-fetcher

# 4. Start PostgreSQL and Redis (if not using Docker)
# macOS:
brew services start postgresql
brew services start redis

# Ubuntu/Debian:
sudo systemctl start postgresql
sudo systemctl start redis-server

# 5. Create database
createdb rss_aggregator

# 6. Set up environment variables
cat > .env << EOF
DATABASE_URL=postgresql://localhost/rss_aggregator
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key-here
PORT=3000
RUST_LOG=info
EOF

# 7. Run server (from root directory)
cargo run
```

### Development Workflow

**With Docker (Hot-reload)**
```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up

# The server will automatically reload on code changes
# Access at http://localhost:3000
```

**Local Development**
```bash
# Install cargo-watch for auto-reload
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run

# Or run tests on change
cargo watch -x test

# Run the main binary (albatross)
cargo watch -x "run -p albatross"
```

### Environment Variables

Create a `.env` file in the project root:

```bash
# Database Configuration
DATABASE_URL=postgresql://user:password@localhost/rss_aggregator

# Redis Configuration (Phase 3+)
REDIS_URL=redis://localhost:6379

# API Configuration
JWT_SECRET=your-secret-key-here-at-least-32-characters
PORT=3000

# Logging
RUST_LOG=info  # Options: trace, debug, info, warn, error

# Optional: External API keys (for Phase 5 features)
# OPENAI_API_KEY=your-key-for-ai-summaries
```

### Verify Installation

```bash
# Check Rust installation
rustc --version
cargo --version

# Check Docker (if using)
docker --version
docker-compose --version

# Run tests
cargo test --workspace

# Check database connection (local)
psql -U postgres -d rss_aggregator -c "SELECT version();"

# Check Redis connection (local)
redis-cli ping  # Should return PONG
```

---

## Key Dependencies by Phase

### Phase 1
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["serde", "v4"] }
quick-xml = "0.31"
chrono = { version = "0.4", features = ["serde"] }
```

### Phase 2
```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
tokio = { version = "1.35", features = ["full", "tracing"] }
```

### Phase 3
```toml
jsonwebtoken = "9.2"
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
tower = "0.4"
tower-governor = "0.1"
bcrypt = "0.15"
```

### Phase 4
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
thiserror = "1.0"
utoipa = { version = "4.1", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "4.0", features = ["axum"] }
config = "0.13"
```

### Phase 5
```toml
criterion = "0.5"
tokio-metrics = "0.3"
flate2 = "1.0"  # For compression
```

---

## Learning Resources

### Essential Reading
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

### Best Practices to Learn
1. **Error Handling**: Never use `unwrap()` in production code
2. **Borrowing**: Understand when to use `&`, `&mut`, and owned values
3. **Async**: Know when to use `spawn` vs `await`, `select!` vs `join!`
4. **Testing**: Write tests as you go, not at the end
5. **Documentation**: Use `cargo doc` and document public APIs
6. **Type Safety**: Leverage Rust's type system (newtypes, enums)
7. **Memory**: Understand `Arc`, `Mutex`, and `Send`/`Sync`

---

## Testing Strategy

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_feed_parsing

# Run integration tests only
cargo test --test '*'

# Run benchmarks (Phase 5)
cargo bench
```

---

## API Examples

### Subscribe to Feed
```bash
curl -X POST http://localhost:3000/api/feeds \
  -H "Content-Type: application/json" \
  -d '{"url": "https://blog.rust-lang.org/feed.xml"}'
```

### Get Articles
```bash
curl "http://localhost:3000/api/articles?limit=10&unread_only=true"
```

### Search Articles
```bash
curl "http://localhost:3000/api/search?q=async+rust"
```

---

## Deployment

### Docker Setup

#### Multi-Stage Production Dockerfile
Create a `Dockerfile` in the project root:

```dockerfile
# Stage 1: Build dependencies (cached layer)
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests (root is the albatross package)
COPY Cargo.toml Cargo.lock ./
COPY models/Cargo.toml ./models/
COPY feed-fetcher/Cargo.toml ./feed-fetcher/

# Create dummy source files to build dependencies (caching optimization)
RUN mkdir -p src models/src feed-fetcher/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > models/src/lib.rs && \
    echo "pub fn dummy() {}" > feed-fetcher/src/lib.rs

# Build dependencies (this layer is cached)
RUN cargo build --release --workspace && \
    rm -rf src models/src feed-fetcher/src

# Copy actual source code
COPY src ./src
COPY models/src ./models/src
COPY feed-fetcher/src ./feed-fetcher/src

# Build the actual application
RUN cargo build --release --bin albatross

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/albatross /usr/local/bin/albatross

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["albatross"]
```

#### .dockerignore
Create a `.dockerignore` file to optimize build context:

```
target/
**/*.rs.bk
**/*.swp
**/*.swo
.git/
.gitignore
.env
.env.*
!.env.example
README.md
docker-compose*.yml
Dockerfile*
.vscode/
.idea/
*.log
```

### Docker Compose (Production)

Create a `docker-compose.yml`:

```yaml
version: '3.8'

services:
  # Main API Server
  api:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: rss-aggregator-api
    ports:
      - "${PORT:-3000}:3000"
    environment:
      - DATABASE_URL=postgresql://${POSTGRES_USER:-postgres}:${POSTGRES_PASSWORD:-password}@postgres:5432/${POSTGRES_DB:-rss_aggregator}
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=${JWT_SECRET:-change-this-in-production}
      - RUST_LOG=${RUST_LOG:-info}
      - PORT=3000
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    networks:
      - rss-network
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  # PostgreSQL Database
  postgres:
    image: postgres:16-alpine
    container_name: rss-aggregator-postgres
    environment:
      - POSTGRES_USER=${POSTGRES_USER:-postgres}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-password}
      - POSTGRES_DB=${POSTGRES_DB:-rss_aggregator}
      - PGDATA=/var/lib/postgresql/data/pgdata
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./database/migrations:/docker-entrypoint-initdb.d
    ports:
      - "${POSTGRES_PORT:-5432}:5432"
    networks:
      - rss-network
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER:-postgres}"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Redis Cache
  redis:
    image: redis:7-alpine
    container_name: rss-aggregator-redis
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    ports:
      - "${REDIS_PORT:-6379}:6379"
    networks:
      - rss-network
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

networks:
  rss-network:
    driver: bridge

volumes:
  postgres_data:
    driver: local
  redis_data:
    driver: local
```

### Docker Compose (Development)

Create a `docker-compose.dev.yml` for development with hot-reload:

```yaml
version: '3.8'

services:
  # Development server with volume mounting
  api-dev:
    image: rust:1.75-slim
    container_name: rss-aggregator-dev
    working_dir: /app
    command: cargo watch -x run
    volumes:
      - .:/app
      - cargo-cache:/usr/local/cargo/registry
      - target-cache:/app/target
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@postgres:5432/rss_aggregator
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=dev-secret-key
      - RUST_LOG=debug
    depends_on:
      - postgres
      - redis
    networks:
      - rss-network

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_PASSWORD: password
      POSTGRES_DB: rss_aggregator
    volumes:
      - postgres_dev_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    networks:
      - rss-network

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    networks:
      - rss-network

  # pgAdmin for database management
  pgadmin:
    image: dpage/pgadmin4:latest
    environment:
      PGADMIN_DEFAULT_EMAIL: admin@example.com
      PGADMIN_DEFAULT_PASSWORD: admin
      PGADMIN_CONFIG_SERVER_MODE: 'False'
    ports:
      - "5050:80"
    networks:
      - rss-network

networks:
  rss-network:
    driver: bridge

volumes:
  postgres_dev_data:
  cargo-cache:
  target-cache:
```

### Docker Commands

```bash
# Build and start all services
docker-compose up -d

# Build with no cache (clean build)
docker-compose build --no-cache

# View logs
docker-compose logs -f api

# Stop all services
docker-compose down

# Stop and remove volumes (clean slate)
docker-compose down -v

# Development mode with hot-reload
docker-compose -f docker-compose.dev.yml up

# Run database migrations
docker-compose exec api /app/migrations/run.sh

# Access PostgreSQL
docker-compose exec postgres psql -U postgres -d rss_aggregator

# Access Redis CLI
docker-compose exec redis redis-cli

# Scale the API (horizontal scaling)
docker-compose up -d --scale api=3

# Monitor resource usage
docker stats

# Prune unused Docker resources
docker system prune -a
```

### Environment Variables

Create a `.env` file (don't commit this):

```bash
# Database
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your_secure_password
POSTGRES_DB=rss_aggregator
POSTGRES_PORT=5432

# Redis
REDIS_PORT=6379
REDIS_PASSWORD=your_redis_password

# API
PORT=3000
JWT_SECRET=your_jwt_secret_key_at_least_32_characters
RUST_LOG=info

# Optional: Monitoring
PROMETHEUS_PORT=9090
GRAFANA_PORT=3001
GRAFANA_USER=admin
GRAFANA_PASSWORD=admin

# Optional: pgAdmin
PGADMIN_EMAIL=admin@example.com
PGADMIN_PASSWORD=admin
PGADMIN_PORT=5050
```

Create a `.env.example` (commit this):

```bash
# Database
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=rss_aggregator
POSTGRES_PORT=5432

# Redis
REDIS_PORT=6379
REDIS_PASSWORD=

# API
PORT=3000
JWT_SECRET=change-this-in-production
RUST_LOG=info
```

### Production Deployment Considerations

1. **Security**
   - Use secrets management (Docker secrets, AWS Secrets Manager, etc.)
   - Never commit `.env` files
   - Run containers as non-root users
   - Use read-only root filesystem where possible
   - Scan images for vulnerabilities: `docker scan rss-aggregator-api`

2. **Performance**
   - Use multi-stage builds to minimize image size
   - Leverage build caching for faster builds
   - Use Alpine-based images where possible
   - Configure resource limits:
     ```yaml
     deploy:
       resources:
         limits:
           cpus: '1'
           memory: 512M
         reservations:
           cpus: '0.5'
           memory: 256M
     ```

3. **Monitoring** (Optional)
   - Add Prometheus for metrics collection
   - Add Grafana for visualization
   - Use Docker health checks
   - Set up log aggregation (ELK stack, Loki, etc.)

4. **Orchestration**
   - For production, consider Kubernetes instead of docker-compose
   - Use managed database services (RDS, Cloud SQL) for reliability
   - Implement CI/CD pipelines
   - Use container registries (Docker Hub, ECR, GCR)

### Kubernetes Deployment (Advanced)

For production Kubernetes deployment, create manifests:

```yaml
# deployment.yaml (simplified example)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rss-aggregator
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rss-aggregator
  template:
    metadata:
      labels:
        app: rss-aggregator
    spec:
      containers:
      - name: api
        image: your-registry/rss-aggregator:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: rss-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: rss-secrets
              key: redis-url
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

### Docker Best Practices

1. **Image Size Optimization**
   ```dockerfile
   # Use specific versions, not 'latest'
   FROM rust:1.75-slim as builder

   # Clean up in same layer to reduce size
   RUN apt-get update && apt-get install -y pkg-config \
       && rm -rf /var/lib/apt/lists/*

   # Use multi-stage builds
   # Final image should only contain runtime dependencies
   ```

2. **Security Hardening**
   ```dockerfile
   # Run as non-root user
   USER appuser

   # Use read-only root filesystem
   # docker run --read-only --tmpfs /tmp myapp

   # Scan for vulnerabilities
   # docker scan rss-aggregator-api
   ```

3. **Build Performance**
   ```bash
   # Use BuildKit for faster builds
   DOCKER_BUILDKIT=1 docker build -t myapp .

   # Cache dependencies separately from source code
   # (shown in the multi-stage Dockerfile above)

   # Use .dockerignore to exclude unnecessary files
   ```

4. **Resource Management**
   ```yaml
   # Set resource limits in docker-compose.yml
   services:
     api:
       deploy:
         resources:
           limits:
             cpus: '1.0'
             memory: 512M
           reservations:
             cpus: '0.5'
             memory: 256M
   ```

### Docker Troubleshooting

#### Common Issues and Solutions

**1. Container fails to start**
```bash
# Check logs
docker-compose logs api

# Inspect container
docker inspect rss-aggregator-api

# Check if ports are already in use
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows
```

**2. Database connection refused**
```bash
# Ensure PostgreSQL is healthy
docker-compose ps

# Check database logs
docker-compose logs postgres

# Verify connection string
docker-compose exec api env | grep DATABASE_URL

# Test connection manually
docker-compose exec postgres psql -U postgres -d rss_aggregator
```

**3. Slow build times**
```bash
# Enable BuildKit
export DOCKER_BUILDKIT=1

# Clear build cache if needed
docker builder prune -a

# Use docker-compose build cache
docker-compose build --parallel
```

**4. Out of disk space**
```bash
# Check Docker disk usage
docker system df

# Clean up unused resources
docker system prune -a --volumes

# Remove specific resources
docker volume prune
docker image prune -a
```

**5. Changes not reflecting in container**
```bash
# Rebuild without cache
docker-compose build --no-cache api

# Ensure you're not using old images
docker-compose down
docker-compose up --build

# For development, ensure volumes are mounted correctly
docker-compose -f docker-compose.dev.yml down -v
docker-compose -f docker-compose.dev.yml up
```

**6. Permission issues with volumes**
```bash
# Check volume ownership
docker-compose exec api ls -la /app

# Fix ownership (if running as root accidentally)
docker-compose exec api chown -R appuser:appuser /app

# Or rebuild with correct USER directive in Dockerfile
```

#### Debugging Tips

```bash
# Execute shell in running container
docker-compose exec api /bin/bash

# Run one-off command
docker-compose run --rm api cargo test

# Check environment variables
docker-compose exec api env

# Monitor resource usage
docker stats

# View real-time logs with timestamps
docker-compose logs -f --timestamps api

# Filter logs
docker-compose logs api | grep ERROR

# Export logs to file
docker-compose logs api > api-logs.txt
```

#### Performance Profiling

```bash
# Check container resource usage
docker stats rss-aggregator-api

# Profile CPU usage
docker top rss-aggregator-api

# Analyze image layers
docker history rss-aggregator-api

# Check build cache utilization
docker-compose build --progress=plain

# Benchmark container startup time
time docker-compose up -d
```

### CI/CD Integration

Example GitHub Actions workflow for Docker:

```yaml
name: Docker Build and Push

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v2

      - name: Login to Docker Hub
        uses: docker/login-action@v2
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}

      - name: Build and push
        uses: docker/build-push-action@v4
        with:
          context: .
          push: true
          tags: |
            yourusername/rss-aggregator:latest
            yourusername/rss-aggregator:${{ github.sha }}
          cache-from: type=registry,ref=yourusername/rss-aggregator:buildcache
          cache-to: type=registry,ref=yourusername/rss-aggregator:buildcache,mode=max

      - name: Run tests
        run: |
          docker-compose -f docker-compose.yml -f docker-compose.test.yml up --abort-on-container-exit
          docker-compose down -v
```

---

## Contributing

This is a learning project, but contributions are welcome! Areas for improvement:
- Additional feed format support
- Mobile app integration
- AI-powered article summaries
- Feed category classification
- Browser extension

---

## Milestones & Checkpoints

- [ ] **Phase 1**: Basic API with RSS fetching (2-3 weeks)
- [ ] **Phase 2**: Database persistence and background updates (2 weeks)
- [ ] **Phase 3**: Authentication, caching, rate limiting (3 weeks)
- [ ] **Phase 4**: Production-ready with tests and docs (2-3 weeks)
- [ ] **Phase 5**: Performance optimization and advanced features (ongoing)

---

## Common Pitfalls & Solutions

### 1. Async Confusion
**Problem**: Blocking operations in async functions
**Solution**: Use `tokio::task::spawn_blocking` for CPU-intensive or blocking I/O

### 2. Clone-Heavy Code
**Problem**: Excessive cloning, `.clone()` everywhere
**Solution**: Use references and `Arc` appropriately

### 3. Error Handling Hell
**Problem**: Nested `match` and `if let`
**Solution**: Use `?` operator, `Result` combinators, `thiserror`

### 4. Database N+1 Queries
**Problem**: Loading related data in loops
**Solution**: Use joins or batch loading

### 5. Unhandled Panics
**Problem**: `unwrap()` causing crashes
**Solution**: Proper error propagation and recovery

---

## Success Criteria

By completing this project, you will:
- ✅ Understand Rust's ownership and borrowing deeply
- ✅ Write idiomatic async Rust with tokio
- ✅ Build production-grade REST APIs with axum
- ✅ Handle errors gracefully with Result and custom error types
- ✅ Work with databases asynchronously (sqlx)
- ✅ Parse and serialize complex data (serde)
- ✅ Implement authentication and authorization
- ✅ Write comprehensive tests (unit, integration, e2e)
- ✅ Deploy Rust applications with Docker
- ✅ Profile and optimize Rust code
- ✅ Contribute to open-source Rust projects

---

## License

MIT or Apache-2.0 (dual license, standard for Rust projects)

---

## Next Steps

1. **Start with Phase 1**: Get a basic server running
2. **Read the Docs**: Don't skip the official documentation
3. **Test Everything**: Write tests as you build features
4. **Ask Questions**: Join Rust Discord, post on r/rust
5. **Iterate**: Don't aim for perfection in early phases
6. **Deploy Early**: Get something running in production (even Phase 2)

**Remember**: This project is designed to be built incrementally. Don't rush through phases. Understanding is more important than completion speed.

Happy coding! 🦀
