# API Testing Guide

This guide shows you how to test the Orders API endpoints.

## Base URL
```
http://localhost:8080/api/v1
```

## Endpoints

### 1. Create Order (POST)

**Endpoint:** `POST /api/v1/orders/orders`

**Description:** Creates a new order in the exchange.

**Request Body:**
```json
{
  "symbol": "BTC/USD",
  "side": "Buy",
  "quantity": "1.5",
  "price": "50000.00",
  "order_type": "Limit"
}
```

**Field Descriptions:**
- `symbol`: Trading pair (e.g., "BTC/USD", "ETH/USD")
- `side`: Order side - "Buy" or "Sell"
- `quantity`: Amount to trade (must be > 0)
- `price`: Price per unit (must be > 0)
- `order_type`: Type of order - "Market", "Limit", "Stop", "StopLimit"

**Example Request:**
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

**Example Response:**
```json
{
  "id": "0ac01023-ebdc-4bba-a225-e3989bdcef26",
  "symbol": "BTC/USD",
  "side": "Buy",
  "quantity": "1.5",
  "price": "50000.00",
  "order_type": "Limit",
  "status": "New",
  "filled_quantity": "0",
  "created_at": "2025-08-08T07:07:21.462284Z"
}
```

### 2. Get Orders (GET)

**Endpoint:** `GET /api/v1/orders/orders`

**Description:** Retrieves all orders with optional filtering.

**Query Parameters:**
- `symbol` (optional): Filter by trading symbol
- `status` (optional): Filter by order status
- `limit` (optional): Number of orders to return
- `offset` (optional): Number of orders to skip

**Example Request:**
```bash
# Get all orders
curl -X GET "http://localhost:8080/api/v1/orders/orders"

# Get orders for specific symbol
curl -X GET "http://localhost:8080/api/v1/orders/orders?symbol=BTC/USD"

# Get orders with limit and offset
curl -X GET "http://localhost:8080/api/v1/orders/orders?limit=10&offset=0"
```

**Example Response:**
```json
[
  {
    "id": "0ac01023-ebdc-4bba-a225-e3989bdcef26",
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "1.5",
    "price": "50000.00",
    "order_type": "Limit",
    "status": "New",
    "filled_quantity": "0",
    "created_at": "2025-08-08T07:07:21.462284Z"
  }
]
```

### 3. Get Order by ID (GET)

**Endpoint:** `GET /api/v1/orders/orders/{id}`

**Description:** Retrieves a specific order by its ID.

**Example Request:**
```bash
curl -X GET "http://localhost:8080/api/v1/orders/orders/0ac01023-ebdc-4bba-a225-e3989bdcef26"
```

### 4. Cancel Order (PUT)

**Endpoint:** `PUT /api/v1/orders/orders/{id}/cancel`

**Description:** Cancels a specific order.

**Example Request:**
```bash
curl -X PUT "http://localhost:8080/api/v1/orders/orders/0ac01023-ebdc-4bba-a225-e3989bdcef26/cancel"
```

### 5. Get Order Trades (GET)

**Endpoint:** `GET /api/v1/orders/orders/{id}/trades`

**Description:** Retrieves all trades for a specific order.

**Example Request:**
```bash
curl -X GET "http://localhost:8080/api/v1/orders/orders/0ac01023-ebdc-4bba-a225-e3989bdcef26/trades"
```

## Testing Examples

### Create Multiple Orders

```bash
# Create a buy order
curl -X POST "http://localhost:8080/api/v1/orders/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "1.0",
    "price": "50000.00",
    "order_type": "Limit"
  }'

# Create a sell order
curl -X POST "http://localhost:8080/api/v1/orders/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "ETH/USD",
    "side": "Sell",
    "quantity": "10.0",
    "price": "3000.00",
    "order_type": "Limit"
  }'

# Create a market order
curl -X POST "http://localhost:8080/api/v1/orders/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "0.5",
    "price": "50000.00",
    "order_type": "Market"
  }'
```

### Query Orders with Filters

```bash
# Get all buy orders
curl -X GET "http://localhost:8080/api/v1/orders/orders?side=Buy"

# Get orders for BTC/USD
curl -X GET "http://localhost:8080/api/v1/orders/orders?symbol=BTC/USD"

# Get first 5 orders
curl -X GET "http://localhost:8080/api/v1/orders/orders?limit=5"
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Request successful
- `201 Created`: Order created successfully
- `400 Bad Request`: Invalid request data
- `404 Not Found`: Order not found
- `500 Internal Server Error`: Server error

## Validation Rules

- `symbol`: Must be between 1 and 20 characters
- `quantity`: Must be greater than 0
- `price`: Must be greater than 0
- `side`: Must be "Buy" or "Sell"
- `order_type`: Must be "Market", "Limit", "Stop", or "StopLimit"

## Testing with Tools

### Using curl
All examples above use curl. Make sure to include the `Content-Type: application/json` header for POST requests.

### Using Postman
1. Set the base URL to `http://localhost:8080/api/v1`
2. Use the endpoint paths as shown above
3. For POST requests, set the body to raw JSON and include the request body

### Using JavaScript/Fetch
```javascript
// Create order
const response = await fetch('http://localhost:8080/api/v1/orders/orders', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    symbol: 'BTC/USD',
    side: 'Buy',
    quantity: '1.5',
    price: '50000.00',
    order_type: 'Limit'
  })
});

const order = await response.json();
console.log(order);
```
