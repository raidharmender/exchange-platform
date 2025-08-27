# Exchange Platform

A modern cryptocurrency exchange platform built with Rust (backend) and React/TypeScript (frontend).

## Table of Contents

1. [Quick Start](#quick-start)
2. [Architecture](#architecture)
3. [Development Setup](#development-setup)
4. [Testing](#testing)
5. [API Documentation](#api-documentation)
6. [Deployment](#deployment)
7. [Troubleshooting](#troubleshooting)
8. [Contributing](#contributing)

## Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker (optional)
- PostgreSQL (optional for development)
- Redis (optional for development)

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

# Access Swagger UI
open http://localhost:8080/swagger-ui
```

## Architecture

### Backend (Rust/Actix-web)
- **Framework**: Actix-web 4.4
- **Database**: PostgreSQL (with SQLx)
- **Cache**: Redis
- **Authentication**: JWT
- **Testing**: Built-in Rust testing framework
- **Features**: Conditional compilation for database/mock modes
- **API Documentation**: Swagger UI with OpenAPI 3.0 specification

### Frontend (React/TypeScript/Vite)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite with HMR (Hot Module Replacement)
- **Testing**: Vitest + Playwright for E2E testing
- **Styling**: Tailwind CSS with PostCSS
- **State Management**: Zustand
- **HTTP Client**: Axios
- **Linting**: ESLint with TypeScript support
- **Type Checking**: Strict TypeScript configuration

## Development Setup

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

## Testing

### Testing Strategy

The Exchange Platform follows a comprehensive testing strategy with a testing pyramid approach:

```
        /\
       /  \
      / E2E \
     /______\
    /        \
   /Integration\
  /____________\
 /              \
/   Unit Tests   \
/________________\
```

### Distribution
- **Unit Tests**: 70% of test coverage
- **Integration Tests**: 20% of test coverage
- **E2E Tests**: 10% of test coverage

### Test Types

#### 1. Unit Tests

**Backend (Rust)**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Order, OrderSide, OrderType};
    use rust_decimal_macros::dec;

    #[test]
    fn test_order_creation() {
        let order = Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Buy,
            quantity: dec!(1.0),
            price: dec!(50000.0),
            order_type: OrderType::Limit,
            status: OrderStatus::New,
            filled_quantity: dec!(0.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(order.symbol, "BTC/USD");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.quantity, dec!(1.0));
    }
}
```

**Frontend (React/TypeScript)**
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { OrderForm } from '../components/OrderForm';

describe('OrderForm', () => {
  it('should create a buy order when form is submitted', async () => {
    const mockCreateOrder = jest.fn();
    render(<OrderForm onCreateOrder={mockCreateOrder} />);

    // Fill form
    fireEvent.change(screen.getByLabelText(/symbol/i), {
      target: { value: 'BTC/USD' },
    });
    fireEvent.change(screen.getByLabelText(/quantity/i), {
      target: { value: '1.0' },
    });
    fireEvent.change(screen.getByLabelText(/price/i), {
      target: { value: '50000' },
    });

    // Submit form
    fireEvent.click(screen.getByRole('button', { name: /place order/i }));

    expect(mockCreateOrder).toHaveBeenCalledWith({
      symbol: 'BTC/USD',
      side: 'buy',
      quantity: '1.0',
      price: '50000',
      orderType: 'limit',
    });
  });
});
```

#### 2. Integration Tests

**API Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use actix_web::{test, web, App};
    use crate::handlers::orders::*;
    use crate::services::order_service::OrderService;

    #[actix_web::test]
    async fn test_create_order_integration() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api/v1").service(create_order))
        ).await;

        let order_request = CreateOrderRequest {
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Buy,
            quantity: dec!(1.0),
            price: dec!(50000.0),
            order_type: OrderType::Limit,
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/orders")
            .set_json(&order_request)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

#### 3. E2E Tests

**Playwright Tests**
```typescript
import { test, expect } from '@playwright/test';

test.describe('Exchange Platform E2E', () => {
  test('should place and execute an order', async ({ page }) => {
    // Navigate to trading page
    await page.goto('/trading');
    
    // Login
    await page.fill('[data-testid="email"]', 'test@example.com');
    await page.fill('[data-testid="password"]', 'password123');
    await page.click('[data-testid="login-button"]');
    
    // Wait for login to complete
    await page.waitForURL('/trading');
    
    // Fill order form
    await page.selectOption('[data-testid="symbol-select"]', 'BTC/USD');
    await page.fill('[data-testid="quantity-input"]', '1.0');
    await page.fill('[data-testid="price-input"]', '50000');
    await page.click('[data-testid="buy-button"]');
    
    // Verify order was created
    await expect(page.locator('[data-testid="order-success"]')).toBeVisible();
    
    // Check order book
    await page.goto('/orderbook');
    await expect(page.locator('[data-testid="order-book"]')).toBeVisible();
  });
});
```

### TDD Approach

The platform follows Test-Driven Development (TDD) with the Red-Green-Refactor cycle:

#### Red Phase - Write Failing Test
```rust
#[test]
fn test_order_matching_algorithm() {
    let mut order_book = OrderBook::new();
    
    // Test that buy orders match with sell orders at the same or better price
    let sell_order = Order::new("BTC/USD", OrderSide::Sell, dec!(1.0), dec!(50000.0));
    order_book.add_order(sell_order);
    
    let buy_order = Order::new("BTC/USD", OrderSide::Buy, dec!(1.0), dec!(50000.0));
    let trades = order_book.add_order(buy_order);
    
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].quantity, dec!(1.0));
}
```

### BDD Approach

The platform also supports Behavior-Driven Development (BDD) with Cucumber feature files:

```gherkin
Feature: Order Placement
  As a trader
  I want to place orders
  So that I can participate in the market

  Background:
    Given I am logged in as a trader
    And I have sufficient balance

  Scenario: Place a limit buy order
    When I place a buy order for 1.0 BTC at $50000
    Then the order should be created with status "open"
    And the order should appear in the order book
```

### Quality Gates

#### Code Quality Metrics
- **Unit Tests**: Minimum 80% coverage
- **Integration Tests**: Minimum 60% coverage
- **E2E Tests**: Critical user journeys covered
- **API Response Time**: < 100ms for 95% of requests
- **Order Processing**: < 10ms for order matching
- **Database Queries**: < 50ms for 95% of queries

## API Documentation

### Swagger UI

The Exchange API includes a complete Swagger UI implementation:

- **Swagger UI**: `http://localhost:8080/swagger-ui` - Interactive API documentation
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json` - Machine-readable specification

### API Endpoints

#### Health Check
```bash
GET /api/v1/health
```

#### Orders Management
```bash
# Get all orders
GET /api/v1/orders/orders

# Create order
POST /api/v1/orders/orders

# Get specific order
GET /api/v1/orders/orders/{id}

# Cancel order
PUT /api/v1/orders/orders/{id}/cancel

# Get order trades
GET /api/v1/orders/orders/{id}/trades
```

### API Testing Examples

#### Create Order
```bash
curl -X POST "http://localhost:8080/api/v1/orders/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "1.5",
    "price": "50000.00",
    "order_type": "Limit"
  }'
```

#### Get Orders
```bash
curl -X GET "http://localhost:8080/api/v1/orders/orders"
curl -X GET "http://localhost:8080/api/v1/orders/orders?symbol=BTC/USD"
```

### Data Models

#### Order Schema
```json
{
  "id": "string (uuid)",
  "user_id": "string (uuid)",
  "symbol": "string",
  "side": "Buy | Sell",
  "quantity": "string",
  "price": "string",
  "order_type": "Market | Limit | Stop | StopLimit",
  "status": "New | Open | PartiallyFilled | Filled | Cancelled | Rejected",
  "filled_quantity": "string",
  "created_at": "string (date-time)",
  "updated_at": "string (date-time)"
}
```

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

#### Initial Setup
```bash
# Create namespace
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

#### Verification
```bash
# Check pod status
kubectl get pods -n exchange

# Check services
kubectl get svc -n exchange

# Test endpoints
curl http://exchange.local/api/v1/health
```

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

## Troubleshooting

### Common Issues

#### Frontend Issues

**Blank Frontend Page**
- Check React Router configuration
- Verify Tailwind CSS setup
- Check component imports
- Check browser console for errors
- Verify Vite configuration

**Dependencies Not Found**
```bash
rm -rf node_modules package-lock.json
npm install
```

**Build Errors**
```bash
npm run build -- --force
```

#### Backend Issues

**Database Connection Failed**
```bash
# Run in mock mode (no database required)
cargo run --no-default-features

# Or run with database (requires PostgreSQL and Redis)
cargo run --features database
```

**Compilation Errors**
```bash
cargo clean
cargo update
cargo check
```

#### Port Conflicts
```bash
# Check what's running on a port
lsof -i :8080
lsof -i :5173

# Kill processes using specific ports
pkill -f "cargo run"
pkill -f "npm run dev"
```

#### Docker Issues
```bash
# Clean up Docker resources
docker system prune -f

# Check Docker disk usage
docker system df
```

### Performance Issues

#### Slow Build Times
```bash
# Backend optimization
cargo build --release

# Frontend optimization
npm run build
```

#### Memory Issues
- Monitor memory usage with `htop` or `top`
- Restart services periodically
- Check for memory leaks in application code

### Network Issues

#### CORS Errors
- Backend is configured with CORS support
- Ensure frontend is making requests to the correct backend URL
- Check that the backend CORS configuration allows the frontend origin

#### Connection Refused
- Verify that services are running on the correct ports
- Check firewall settings
- Ensure services are binding to the correct interfaces

### Debugging Commands
```bash
# Backend debugging
RUST_LOG=debug cargo run --no-default-features
tail -f backend.log

# Frontend debugging
npm run dev -- --debug
# Check browser console (F12)

# Health checks
curl http://localhost:8080/api/v1/health
curl http://localhost:5173
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

### Pre-commit Hooks
```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run tests
cd backend && cargo test
if [ $? -ne 0 ]; then
    echo "Backend tests failed"
    exit 1
fi

cd ../frontend && npm test
if [ $? -ne 0 ]; then
    echo "Frontend tests failed"
    exit 1
fi

# Run linting
cd ../backend && cargo clippy
if [ $? -ne 0 ]; then
    echo "Backend linting failed"
    exit 1
fi

cd ../frontend && npm run lint
if [ $? -ne 0 ]; then
    echo "Frontend linting failed"
    exit 1
fi
```

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support and questions:
- Check the troubleshooting section above
- Open an [issue](../../issues)
- Contact the development team at dharmender.rai@yahoo.com

## Roadmap

- [ ] Real-time order book updates
- [ ] WebSocket support
- [ ] Advanced order types
- [ ] User authentication
- [ ] Admin dashboard
- [ ] Mobile app
- [ ] Advanced analytics
- [ ] Multi-language support