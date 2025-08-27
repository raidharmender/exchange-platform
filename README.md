# Exchange Platform

A modern cryptocurrency exchange platform built with Rust (backend) and React/TypeScript (frontend).

## Quick Start

For detailed setup instructions, see the [Development Guide](docs/Development.md#quick-start).

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker (optional)

### Backend Setup
```bash
cd backend
cargo run --no-default-features  # Mock mode (no database required)
```

### Frontend Setup
```bash
cd frontend
npm install
npm run dev
```

### Verify Setup
```bash
curl http://localhost:8080/api/v1/health  # Backend
curl http://localhost:5173                # Frontend
```

## Architecture

For detailed architecture information, see the [Development Guide](docs/Development.md#architecture).

### Backend (Rust/Actix-web)
- **Framework**: Actix-web 4.4
- **Database**: PostgreSQL (with SQLx)
- **Cache**: Redis
- **Authentication**: JWT
- **Testing**: Built-in Rust testing framework
- **Features**: Conditional compilation for database/mock modes

### Frontend (React/TypeScript/Vite)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite with HMR (Hot Module Replacement)
- **Testing**: Vitest + Playwright for E2E testing
- **Styling**: Tailwind CSS with PostCSS
- **State Management**: Zustand
- **HTTP Client**: Axios
- **Linting**: ESLint with TypeScript support
- **Type Checking**: Strict TypeScript configuration

## Testing

For comprehensive testing information, see the [Testing Guide](docs/Testing.md).

### Quick Test Commands
```bash
# Backend
cargo test --no-default-features

# Frontend
npm test
npm run test:e2e
```

## Documentation

- [Development Guide](docs/Development.md) - Comprehensive development guide
- [Runbook & Troubleshooting](docs/Runbook.md) - Operational procedures and troubleshooting guide
- [Testing Guide](docs/Testing.md) - Comprehensive testing strategy and API testing
- [Swagger Documentation](docs/Swagger.md) - API documentation and examples

## Development

For detailed development procedures, see the [Development Guide](docs/Development.md#development-workflow).

### Quick Development Commands
```bash
# Backend
cargo run --no-default-features  # Mock mode
cargo fmt                        # Format code
cargo clippy                     # Lint code

# Frontend
npm run dev                      # Start dev server
npm run lint                     # Lint code
npm test                        # Run tests
```

## Docker

For detailed Docker configurations, see the [Development Guide](docs/Development.md#deployment).

### Quick Docker Commands
```bash
# Build and run backend
docker build -t exchange-api ./backend
docker run -p 8080:8080 exchange-api

# Build and run frontend
docker build -t exchange-frontend ./frontend
docker run -p 80:80 exchange-frontend
```

## Kubernetes

For detailed Kubernetes deployment procedures, see the [Runbook](docs/Runbook.md#deployment-procedures).

### Quick K8S Commands
```bash
# Deploy all components
kubectl apply -f kubernetes/

# Check deployment status
kubectl get pods -n exchange
```

## API Endpoints

For detailed API documentation and testing, see:
- [Swagger Documentation](docs/Swagger.md) - Interactive API documentation
- [Testing Guide](docs/Testing.md#api-testing-guide) - API testing examples

### Quick API Test
```bash
# Health check
curl http://localhost:8080/api/v1/health

# Create order
curl -X POST http://localhost:8080/api/v1/orders/orders \
  -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USD","side":"Buy","quantity":"1.0","price":"50000.00","order_type":"Limit"}'
```

## Troubleshooting

For comprehensive troubleshooting information, see the [Runbook & Troubleshooting Guide](docs/Runbook.md#troubleshooting).

### Quick Fixes
```bash
# Backend issues
cargo clean && cargo run --no-default-features

# Frontend issues
rm -rf node_modules package-lock.json && npm install
```

## How can you contribute

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support and questions:
- Check the [documentation](docs/)
- Open an [issue](../../issues)
- Contact the development team

## Roadmap

- [ ] Real-time order book updates
- [ ] WebSocket support
- [ ] Advanced order types
- [ ] User authentication
- [ ] Admin dashboard
- [ ] Mobile app
- [ ] Advanced analytics
- [ ] Multi-language support

## Anything else
Please send me mail at dharmender.rai@yahoo.com