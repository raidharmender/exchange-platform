# Exchange Platform

A modern cryptocurrency exchange platform built with Rust (backend) and React/TypeScript (frontend).

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker (optional)

### Backend Setup
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

### Frontend Setup
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

### Verify Setup
```bash
# Test backend
curl http://localhost:8080/api/v1/health

# Test frontend
curl http://localhost:5173
```

## 🏗️ Architecture

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

## 🧪 Testing

### Backend Tests
```bash
# Run all tests
cargo test

# Run tests without database
cargo test --no-default-features

# Run tests with output
cargo test -- --nocapture
```

### Frontend Tests
```bash
# Run unit tests
npm test

# Run tests in watch mode
npm run test:ui

# Run end-to-end tests
npm run test:e2e
```

## 📚 Documentation

- [Development Guide](docs/DEVELOPMENT.md) - Comprehensive development guide
- [Runbook](docs/Runbook.md) - Operational procedures and troubleshooting
- [Testing Strategy](docs/Testing%20Strategy.md) - Testing approach and procedures

## 🔧 Development

### Backend Development
```bash
# Mock mode (no database)
cargo run --no-default-features

# With database (requires PostgreSQL and Redis)
cargo run --features database

# Format code
cargo fmt

# Check code
cargo check

# Clippy (linting)
cargo clippy
```

### Frontend Development
```bash
# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Lint code
npm run lint

# Type check
npx tsc --noEmit
```

## 🐳 Docker

### Backend
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

### Frontend
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

## ☸️ Kubernetes

The platform includes Kubernetes manifests for deployment:

```bash
# Apply namespace
kubectl apply -f kubernetes/namespace.yml

# Apply secrets and configmaps
kubectl apply -f kubernetes/secrets.yml
kubectl apply -f kubernetes/configmap.yml

# Deploy infrastructure
kubectl apply -f kubernetes/postgres-deployment.yml
kubectl apply -f kubernetes/redis-deployment.yml

# Deploy applications
kubectl apply -f kubernetes/api-deployment.yml
kubectl apply -f kubernetes/frontend-deployment.yml

# Deploy ingress
kubectl apply -f kubernetes/ingress.yml
```

## 🔍 API Endpoints

### Health Check
```bash
GET /api/v1/health
```

### Orders
```bash
# Create order
POST /api/v1/orders
{
  "symbol": "BTC/USD",
  "side": "Buy",
  "quantity": "1.0",
  "price": "50000.00",
  "order_type": "Limit"
}

# Get orders
GET /api/v1/orders

# Get specific order
GET /api/v1/orders/{id}

# Cancel order
PUT /api/v1/orders/{id}/cancel

# Get order trades
GET /api/v1/orders/{id}/trades
```

## 🚨 Troubleshooting

### Common Issues

#### Backend Issues
1. **Database Connection Failed**
   - Check if PostgreSQL is running
   - Verify DATABASE_URL in .env
   - Use mock mode: `cargo run --no-default-features`

2. **Compilation Errors**
   - Run `cargo clean` and try again
   - Check Rust version: `rustc --version`
   - Update dependencies: `cargo update`

#### Frontend Issues
1. **Dependencies Not Found**
   - Delete `node_modules` and `package-lock.json`
   - Run `npm install`

2. **Build Errors**
   - Check Node.js version: `node --version`
   - Clear cache: `npm run build -- --force`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

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

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For support and questions:
- Check the [documentation](docs/)
- Open an [issue](../../issues)
- Contact the development team

## 🗺️ Roadmap

- [ ] Real-time order book updates
- [ ] WebSocket support
- [ ] Advanced order types
- [ ] User authentication
- [ ] Admin dashboard
- [ ] Mobile app
- [ ] Advanced analytics
- [ ] Multi-language support
