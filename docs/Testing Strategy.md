# Exchange Platform - Testing Strategy

## Table of Contents

1. [Overview](#overview)
2. [Testing Pyramid](#testing-pyramid)
3. [Test Types](#test-types)
4. [TDD Approach](#tdd-approach)
5. [BDD Approach](#bdd-approach)
6. [Test Implementation](#test-implementation)
7. [Test Automation](#test-automation)
8. [Quality Gates](#quality-gates)

## Overview

This document outlines the comprehensive testing strategy for the Exchange Platform, covering Test-Driven Development (TDD), Behavior-Driven Development (BDD), and various testing approaches to ensure high-quality, reliable software.

### Testing Goals
- **Quality Assurance**: Ensure software meets functional and non-functional requirements
- **Risk Mitigation**: Identify and fix issues early in the development cycle
- **Regression Prevention**: Maintain software quality across releases
- **Performance Validation**: Ensure system meets performance requirements
- **Security Verification**: Validate security measures and identify vulnerabilities

## Testing Pyramid

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

## Test Types

### 1. Unit Tests

#### 1.1 Backend (Rust)
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

    #[test]
    fn test_order_book_matching() {
        let mut order_book = OrderBookService::new();
        
        // Add a sell order
        let sell_order = create_test_order(OrderSide::Sell, dec!(50000.0), dec!(1.0));
        order_book.add_order(&sell_order).await.unwrap();
        
        // Add a buy order that should match
        let buy_order = create_test_order(OrderSide::Buy, dec!(50000.0), dec!(1.0));
        let trades = order_book.add_order(&buy_order).await.unwrap();
        
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, dec!(1.0));
        assert_eq!(trades[0].price, dec!(50000.0));
    }
}
```

#### 1.2 Frontend (React/TypeScript)
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

### 2. Integration Tests

#### 2.1 API Integration Tests
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

#### 2.2 Database Integration Tests
```rust
#[cfg(test)]
mod database_tests {
    use sqlx::PgPool;
    use crate::models::Order;

    #[sqlx::test]
    async fn test_order_persistence() {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        
        let order = Order {
            // ... order data
        };

        let saved_order = sqlx::query_as!(
            Order,
            "INSERT INTO orders (user_id, symbol, side, quantity, price, order_type, status) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING *",
            order.user_id,
            order.symbol,
            order.side as OrderSide,
            order.quantity,
            order.price,
            order.order_type as OrderType,
            order.status as OrderStatus
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(saved_order.symbol, order.symbol);
        assert_eq!(saved_order.side, order.side);
    }
}
```

### 3. E2E Tests

#### 3.1 Playwright Tests
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

## TDD Approach

### 1. TDD Cycle (Red-Green-Refactor)

#### 1.1 Red Phase - Write Failing Test
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

#### 1.2 Green Phase - Write Implementation
```rust
impl OrderBook {
    pub fn add_order(&mut self, order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        
        match order.side {
            OrderSide::Buy => {
                // Match with existing sell orders
                if let Some(sell_order) = self.asks.first_mut() {
                    if sell_order.price <= order.price {
                        let trade = Trade::new(
                            sell_order.id,
                            order.id,
                            order.quantity.min(sell_order.quantity),
                            sell_order.price,
                        );
                        trades.push(trade);
                    }
                }
            }
            OrderSide::Sell => {
                // Match with existing buy orders
                if let Some(buy_order) = self.bids.last_mut() {
                    if buy_order.price >= order.price {
                        let trade = Trade::new(
                            buy_order.id,
                            order.id,
                            order.quantity.min(buy_order.quantity),
                            buy_order.price,
                        );
                        trades.push(trade);
                    }
                }
            }
        }
        
        trades
    }
}
```

#### 1.3 Refactor Phase - Improve Code
```rust
impl OrderBook {
    pub fn add_order(&mut self, order: Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        
        match order.side {
            OrderSide::Buy => trades.extend(self.match_buy_order(&order)),
            OrderSide::Sell => trades.extend(self.match_sell_order(&order)),
        }
        
        // Add remaining order to book
        if order.quantity > order.filled_quantity {
            self.add_to_book(order);
        }
        
        trades
    }
    
    fn match_buy_order(&mut self, buy_order: &Order) -> Vec<Trade> {
        // Implementation
    }
    
    fn match_sell_order(&mut self, sell_order: &Order) -> Vec<Trade> {
        // Implementation
    }
}
```

### 2. TDD Best Practices

#### 2.1 Test Naming
```rust
#[test]
fn should_match_buy_order_with_existing_sell_order_at_same_price() {
    // Test implementation
}

#[test]
fn should_not_match_buy_order_when_sell_price_is_higher() {
    // Test implementation
}

#[test]
fn should_partially_fill_order_when_quantities_dont_match() {
    // Test implementation
}
```

#### 2.2 Test Organization
```rust
#[cfg(test)]
mod order_book_tests {
    use super::*;
    
    mod matching {
        use super::*;
        
        #[test]
        fn test_buy_sell_matching() {
            // Tests for buy/sell matching
        }
        
        #[test]
        fn test_price_time_priority() {
            // Tests for price-time priority
        }
    }
    
    mod order_management {
        use super::*;
        
        #[test]
        fn test_order_cancellation() {
            // Tests for order cancellation
        }
        
        #[test]
        fn test_order_modification() {
            // Tests for order modification
        }
    }
}
```

## BDD Approach

### 1. Cucumber Feature Files

#### 1.1 Order Placement Feature
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

  Scenario: Place a market buy order
    When I place a market buy order for 1.0 BTC
    Then the order should be executed immediately
    And I should receive a trade confirmation

  Scenario: Place an order with insufficient balance
    When I place a buy order for 1000.0 BTC at $50000
    Then the order should be rejected
    And I should see an "insufficient balance" error
```

#### 1.2 Order Matching Feature
```gherkin
Feature: Order Matching
  As a trading engine
  I want to match orders
  So that trades can be executed

  Scenario: Match buy and sell orders at same price
    Given there is a sell order for 1.0 BTC at $50000
    When I place a buy order for 1.0 BTC at $50000
    Then a trade should be executed for 1.0 BTC at $50000
    And both orders should be marked as "filled"

  Scenario: Partial order matching
    Given there is a sell order for 2.0 BTC at $50000
    When I place a buy order for 1.0 BTC at $50000
    Then a trade should be executed for 1.0 BTC at $50000
    And the sell order should be partially filled
    And the buy order should be marked as "filled"
```

### 2. BDD Implementation

#### 2.1 Step Definitions (Rust)
```rust
use cucumber::{given, when, then};

#[given("I am logged in as a trader")]
async fn logged_in_as_trader(world: &mut MyWorld) {
    world.user = Some(create_test_user().await);
    world.token = Some(authenticate_user(&world.user.unwrap()).await);
}

#[when("I place a buy order for {quantity} {symbol} at ${price}")]
async fn place_buy_order(world: &mut MyWorld, quantity: f64, symbol: String, price: f64) {
    let order_request = CreateOrderRequest {
        symbol,
        side: OrderSide::Buy,
        quantity: Decimal::from_f64(quantity).unwrap(),
        price: Decimal::from_f64(price).unwrap(),
        order_type: OrderType::Limit,
    };
    
    world.response = Some(
        world.client
            .post("/api/v1/orders")
            .json(&order_request)
            .send()
            .await
            .unwrap()
    );
}

#[then("the order should be created with status {status}")]
async fn order_should_be_created_with_status(world: &mut MyWorld, status: String) {
    let response = world.response.as_ref().unwrap();
    assert!(response.status().is_success());
    
    let order: OrderResponse = response.json().await.unwrap();
    assert_eq!(order.status.to_string(), status);
}
```

#### 2.2 Step Definitions (TypeScript)
```typescript
import { Given, When, Then } from '@cucumber/cucumber';
import { expect } from 'chai';

Given('I am logged in as a trader', async function() {
    this.user = await createTestUser();
    this.token = await authenticateUser(this.user);
});

When('I place a buy order for {float} {string} at ${float}', async function(quantity: number, symbol: string, price: number) {
    const orderRequest = {
        symbol,
        side: 'buy',
        quantity: quantity.toString(),
        price: price.toString(),
        orderType: 'limit'
    };
    
    this.response = await this.client.post('/api/v1/orders', orderRequest);
});

Then('the order should be created with status {string}', async function(status: string) {
    expect(this.response.status).to.equal(200);
    const order = this.response.data;
    expect(order.status).to.equal(status);
});
```

## Test Implementation

### 1. Test Environment Setup

#### 1.1 Docker Compose for Testing
```yaml
# docker-compose.test.yml
version: '3.8'
services:
  postgres-test:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: exchange_test
      POSTGRES_USER: test_user
      POSTGRES_PASSWORD: test_password
    ports:
      - "5433:5432"

  redis-test:
    image: redis:7-alpine
    ports:
      - "6380:6379"

  api-test:
    build:
      context: ./backend
      dockerfile: Dockerfile.test
    environment:
      DATABASE_URL: postgresql://test_user:test_password@postgres-test:5432/exchange_test
      REDIS_URL: redis://redis-test:6379
    depends_on:
      - postgres-test
      - redis-test
```

#### 1.2 Test Configuration
```rust
// tests/common/mod.rs
use sqlx::PgPool;
use redis::Client;

pub async fn setup_test_database() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test_user:test_password@localhost:5433/exchange_test".to_string());
    
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    pool
}

pub async fn setup_test_redis() -> Client {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6380".to_string());
    
    Client::open(redis_url).unwrap()
}
```

### 2. Test Data Management

#### 2.1 Factory Pattern
```rust
// tests/factories.rs
pub struct OrderFactory;

impl OrderFactory {
    pub fn new_buy_order() -> Order {
        Order {
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
        }
    }
    
    pub fn new_sell_order() -> Order {
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Sell,
            quantity: dec!(1.0),
            price: dec!(50000.0),
            order_type: OrderType::Limit,
            status: OrderStatus::New,
            filled_quantity: dec!(0.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
```

## Test Automation

### 1. CI/CD Pipeline

#### 1.1 GitHub Actions
```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Run backend tests
      run: |
        cd backend
        cargo test --verbose
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: frontend/package-lock.json
    
    - name: Run frontend tests
      run: |
        cd frontend
        npm ci
        npm test
    
    - name: Run E2E tests
      run: |
        cd frontend
        npm run test:e2e
```

### 2. Test Reporting

#### 2.1 Coverage Reports
```yaml
# .github/workflows/coverage.yml
name: Coverage

on: [push]

jobs:
  coverage:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Install cargo-tarpaulin
      run: cargo install cargo-tarpaulin
    
    - name: Generate coverage report
      run: |
        cd backend
        cargo tarpaulin --out Html
    
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: backend/tarpaulin-report.html
```

## Quality Gates

### 1. Code Quality Metrics

#### 1.1 Coverage Requirements
- **Unit Tests**: Minimum 80% coverage
- **Integration Tests**: Minimum 60% coverage
- **E2E Tests**: Critical user journeys covered

#### 1.2 Performance Requirements
- **API Response Time**: < 100ms for 95% of requests
- **Order Processing**: < 10ms for order matching
- **Database Queries**: < 50ms for 95% of queries

#### 1.3 Security Requirements
- **Vulnerability Scanning**: No critical vulnerabilities
- **Dependency Updates**: All dependencies up to date
- **Security Tests**: Authentication and authorization tested

### 2. Quality Gates Implementation

#### 2.1 Pre-commit Hooks
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

#### 2.2 Pull Request Checks
```yaml
# .github/workflows/pr-checks.yml
name: PR Checks

on: [pull_request]

jobs:
  checks:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Run all tests
      run: |
        # Backend tests
        cd backend
        cargo test --verbose
        cargo clippy
        cargo audit
        
        # Frontend tests
        cd ../frontend
        npm ci
        npm test
        npm run lint
        npm run build
    
    - name: Check coverage
      run: |
        cd backend
        cargo tarpaulin --out Xml
        # Fail if coverage < 80%
```

## Conclusion

This testing strategy ensures comprehensive coverage of the Exchange Platform through multiple testing approaches:

1. **TDD** ensures code quality and design
2. **BDD** ensures business requirements are met
3. **Automated testing** ensures consistent quality
4. **Quality gates** ensure standards are maintained

The combination of these approaches provides confidence in the software's reliability, performance, and security. 