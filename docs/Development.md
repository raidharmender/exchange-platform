# Exchange Platform - Development Guide

## Overview

This document provides a comprehensive guide for developing the Exchange Platform, including setup, testing, and deployment procedures.

## Architecture

### Backend (Rust/Actix-web)
- **Framework**: Actix-web 4.4
- **Database**: PostgreSQL (with SQLx)
- **Cache**: Redis
- **Authentication**: JWT
- **Testing**: Built-in Rust testing framework
- **Features**: Conditional compilation for database/mock modes

### Frontend (React/TypeScript/Vite)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **Testing**: Vitest + Playwright
- **Styling**: Tailwind CSS
- **State Management**: Zustand
- **HTTP Client**: Axios

## Quick Start

### 1. Clone and Setup
```bash
git clone <repository-url>
cd exchange_docker
```

### 2. Backend Setup
```bash
cd backend

# Create environment file
cat > .env << EOF
DATABASE_URL=postgresql://user:password@localhost:5432/exchange
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-here-change-in-production
RUST_LOG=info
EOF

# Run in mock mode (no database required)
cargo run --no-default-features
```

### 3. Frontend Setup
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

### 4. Verify Setup
```bash
# Test backend
curl http://localhost:8080/api/v1/health

# Test frontend
curl http://localhost:5173
```

## Development Workflow

### Backend Development

#### Running in Development Mode
```bash
# Mock mode (no database)
cargo run --no-default-features

# With database (requires PostgreSQL and Redis)
cargo run --features database
```

#### Testing
```bash
# Run all tests
cargo test

# Run tests without database
cargo test --no-default-features

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

#### Code Quality
```bash
# Format code
cargo fmt

# Check code
cargo check

# Clippy (linting)
cargo clippy
```

### Frontend Development

#### Running in Development Mode
```bash
# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

#### Testing
```bash
# Run unit tests
npm test

# Run tests in watch mode
npm run test:ui

# Run end-to-end tests
npm run test:e2e

# Run tests with coverage
npm run test -- --coverage
```

#### Code Quality
```bash
# Lint code
npm run lint

# Fix linting issues
npm run lint -- --fix

# Type check
npx tsc --noEmit
```

## Testing Strategy

For comprehensive testing information, see the [Testing Guide](Testing.md).

### Quick Testing Overview

#### Backend Testing
- Unit tests in `src/` files with `#[cfg(test)]` modules
- Integration tests in `tests/` directory
- Use `cargo test --no-default-features` for mock mode

#### Frontend Testing
- Unit tests with Vitest and React Testing Library
- E2E tests with Playwright
- Use `npm test` and `npm run test:e2e`

## API Development

For detailed API documentation and testing, see:
- [Swagger Documentation](Swagger.md) - Interactive API documentation
- [Testing Guide](Testing.md#api-testing-guide) - API testing examples

### Quick API Reference
- Health Check: `GET /api/v1/health`
- Orders: `GET/POST /api/v1/orders/orders`
- Order Management: `GET/PUT /api/v1/orders/orders/{id}`
- Trades: `GET /api/v1/orders/orders/{id}/trades`

## Database Schema

### Tables

#### Users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Orders
```sql
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    symbol VARCHAR(20) NOT NULL,
    side order_side NOT NULL,
    quantity DECIMAL NOT NULL,
    price DECIMAL NOT NULL,
    order_type order_type NOT NULL,
    status order_status NOT NULL DEFAULT 'new',
    filled_quantity DECIMAL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Trades
```sql
CREATE TABLE trades (
    id UUID PRIMARY KEY,
    order_id UUID REFERENCES orders(id),
    symbol VARCHAR(20) NOT NULL,
    quantity DECIMAL NOT NULL,
    price DECIMAL NOT NULL,
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## Configuration

### Environment Variables

#### Backend
```bash
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/exchange

# Redis
REDIS_URL=redis://localhost:6379

# JWT
JWT_SECRET=your-super-secret-jwt-key-here-change-in-production

# Logging
RUST_LOG=info

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

#### Frontend
```bash
# API Base URL
VITE_API_BASE_URL=http://localhost:8080
```

## Deployment

### Docker

#### Backend
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/exchange-api /usr/local/bin/
EXPOSE 8080
CMD ["exchange-api"]
```

#### Frontend
```dockerfile
FROM node:18-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

### Kubernetes

#### Backend Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: exchange-api
  namespace: exchange
spec:
  replicas: 3
  selector:
    matchLabels:
      app: exchange-api
  template:
    metadata:
      labels:
        app: exchange-api
    spec:
      containers:
      - name: exchange-api
        image: exchange/api:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: exchange-secrets
              key: database-url
```

## Troubleshooting

For comprehensive troubleshooting information, see the [Runbook & Troubleshooting Guide](Runbook.md#troubleshooting).

### Quick Debugging Commands
```bash
# Backend debugging
RUST_LOG=debug cargo run --no-default-features
tail -f backend.log

# Frontend debugging
npm run dev -- --debug
# Check browser console (F12)
```

## Contributing

### Code Style

#### Rust
- Use `cargo fmt` for formatting
- Follow Rust conventions
- Use meaningful variable names
- Add documentation comments

#### TypeScript/React
- Use `npm run lint` for linting
- Follow React best practices
- Use TypeScript strictly
- Add JSDoc comments

### Git Workflow
1. Create feature branch
2. Make changes
3. Run tests
4. Submit pull request
5. Code review
6. Merge to main

## Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix-web Documentation](https://actix.rs/docs/)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)

### Tools
- [Rust Playground](https://play.rust-lang.org/)
- [TypeScript Playground](https://www.typescriptlang.org/play/)
- [Postman](https://www.postman.com/) - API testing

### Community
- [Rust Community](https://www.rust-lang.org/community)
- [React Community](https://reactjs.org/community/support.html)
