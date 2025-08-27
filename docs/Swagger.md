# Swagger UI Implementation

This document describes the Swagger UI implementation for the Exchange API.

## Overview

The Exchange API now includes a complete Swagger UI implementation that provides:
- Interactive API documentation
- Real-time API testing capabilities
- OpenAPI 3.0 specification
- Request/response examples
- Comprehensive endpoint documentation

## Access Points

### Swagger UI Interface
- **URL**: `http://localhost:8080/swagger-ui`
- **Description**: Interactive web interface for API documentation and testing

### OpenAPI Specification
- **URL**: `http://localhost:8080/api-docs/openapi.json`
- **Description**: Machine-readable OpenAPI 3.0 specification

## Implementation Details

### Architecture
- Uses external Swagger UI CDN for simplicity and reliability
- Serves OpenAPI specification from `backend/openapi.json`
- Integrated with Actix-web framework
- Supports CORS for cross-origin requests

### File Structure
```
backend/
├── src/
│   └── main.rs              # Swagger UI routes and configuration
├── openapi.json             # OpenAPI 3.0 specification
└── Cargo.toml               # Dependencies
```

### Key Components

#### 1. OpenAPI Specification (`openapi.json`)
- Complete API documentation in OpenAPI 3.0 format
- Includes all endpoints, request/response schemas, and examples
- Structured with proper tags and descriptions

#### 2. Swagger UI Route (`main.rs`)
```rust
#[get("/swagger-ui")]
async fn swagger_ui() -> HttpResponse {
    // Returns HTML page with Swagger UI
}
```

#### 3. OpenAPI Spec Route (`main.rs`)
```rust
#[get("/api-docs/openapi.json")]
async fn openapi_spec() -> HttpResponse {
    // Returns OpenAPI specification JSON
}
```

## API Endpoints Documented

### Health Check
- **Endpoint**: `GET /api/v1/health`
- **Description**: Check if the service is healthy
- **Response**: Health status with timestamp and version

### Orders Management
- **Endpoint**: `GET /api/v1/orders/orders`
- **Description**: Retrieve all orders with optional filtering
- **Parameters**: `symbol`, `status`, `limit`, `offset`

- **Endpoint**: `POST /api/v1/orders/orders`
- **Description**: Create a new order
- **Request Body**: `CreateOrderRequest` schema

- **Endpoint**: `GET /api/v1/orders/orders/{id}`
- **Description**: Retrieve a specific order by ID
- **Parameters**: `id` (UUID)

- **Endpoint**: `PUT /api/v1/orders/orders/{id}/cancel`
- **Description**: Cancel an existing order
- **Parameters**: `id` (UUID)

- **Endpoint**: `GET /api/v1/orders/orders/{id}/trades`
- **Description**: Get trades for a specific order
- **Parameters**: `id` (UUID)

## Data Models

### Order Schema
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

### CreateOrderRequest Schema
```json
{
  "symbol": "string",
  "side": "Buy | Sell",
  "quantity": "string",
  "price": "string",
  "order_type": "Market | Limit | Stop | StopLimit"
}
```

### Trade Schema
```json
{
  "id": "string (uuid)",
  "order_id": "string (uuid)",
  "symbol": "string",
  "quantity": "string",
  "price": "string",
  "executed_at": "string (date-time)"
}
```

## Usage Examples

For detailed API testing examples, see the [Testing Guide](Testing.md#api-testing-guide).

### Quick Testing with Swagger UI

1. **Access Swagger UI**: `http://localhost:8080/swagger-ui`
2. **Test Health Check**: Navigate to "Health" section → "GET /api/v1/health" → "Try it out"
3. **Create Order**: Navigate to "Orders" section → "POST /api/v1/orders/orders" → Enter JSON body → "Execute"
4. **Get Orders**: Navigate to "GET /api/v1/orders/orders" → "Try it out" → "Execute"

### Quick Testing with curl

```bash
# Health check
curl -X GET "http://localhost:8080/api/v1/health"

# Create order
curl -X POST "http://localhost:8080/api/v1/orders/orders" \
  -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USD","side":"Buy","quantity":"1.5","price":"50000.00","order_type":"Limit"}'

# Get orders
curl -X GET "http://localhost:8080/api/v1/orders/orders"
```

## Configuration

### Development Setup
```bash
# Start backend with Swagger UI
cargo run --no-default-features

# Access Swagger UI
open http://localhost:8080/swagger-ui
```

### Production Considerations
- Swagger UI is currently configured for development
- For production, consider:
  - Disabling Swagger UI in production builds
  - Using environment variables to control access
  - Implementing authentication for API documentation
  - Using a CDN for Swagger UI assets

## Troubleshooting

### Common Issues

1. **Swagger UI not loading**:
   - Check if the backend is running
   - Verify the URL: `http://localhost:8080/swagger-ui`
   - Check browser console for errors

2. **OpenAPI spec not found**:
   - Verify the file `backend/openapi.json` exists
   - Check the route configuration in `main.rs`

3. **CORS issues**:
   - Backend is configured with CORS support
   - Check browser console for CORS errors

### Debugging
```bash
# Check if backend is running
curl http://localhost:8080/api/v1/health

# Check Swagger UI
curl http://localhost:8080/swagger-ui

# Check OpenAPI spec
curl http://localhost:8080/api-docs/openapi.json
```

## Future Enhancements

1. **Authentication**: Add authentication to Swagger UI
2. **Custom Styling**: Customize Swagger UI appearance
3. **Additional Endpoints**: Document more API endpoints
4. **Examples**: Add more request/response examples
5. **Testing**: Add automated testing for Swagger UI

## References

- [OpenAPI 3.0 Specification](https://swagger.io/specification/)
- [Swagger UI Documentation](https://swagger.io/tools/swagger-ui/)
- [Actix-web Documentation](https://actix.rs/docs/)
- [Rust Documentation](https://doc.rust-lang.org/)
