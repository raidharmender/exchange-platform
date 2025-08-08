# Troubleshooting Guide

This document provides solutions to common issues encountered during development and deployment.

## Frontend Issues

### Blank Frontend Page

**Problem**: The React frontend displays a blank page even though the Vite development server is running.

**Root Cause**: The issue was caused by React Router and component import problems, combined with missing Tailwind CSS configuration.

**Solution**:

1. **Check React Router Configuration**:
   - Ensure all imported components exist and are properly exported
   - Verify that the routing configuration is correct
   - Check for any JavaScript errors in the browser console

2. **Verify Tailwind CSS Setup**:
   ```bash
   # Check if Tailwind CSS is properly configured
   ls frontend/tailwind.config.js
   ls frontend/postcss.config.js
   ```

3. **Check Component Imports**:
   - Ensure all components referenced in `App.tsx` exist
   - Verify that the import paths are correct
   - Check for any TypeScript compilation errors

4. **Browser Console Debugging**:
   - Open browser developer tools (F12)
   - Check the Console tab for JavaScript errors
   - Look for any React-related error messages

5. **Vite Configuration**:
   - Ensure `vite.config.ts` is properly configured
   - Check that the React plugin is enabled
   - Verify that the development server is running on the correct port

**Prevention**:
- Always check browser console for errors when the frontend appears blank
- Ensure all dependencies are properly installed
- Test component imports individually
- Use TypeScript for better error detection

## Backend Issues

### Database Connection Issues

**Problem**: Backend fails to start due to database connection errors.

**Solution**:
```bash
# Run in mock mode (no database required)
cargo run --no-default-features

# Or run with database (requires PostgreSQL and Redis)
cargo run --features database
```

### Swagger UI Implementation

**Problem**: Need to provide API documentation for testing.

**Solution**: The backend now includes a complete Swagger UI implementation.

**Features**:
- Interactive API documentation
- Request/response examples
- Real-time API testing
- OpenAPI 3.0 specification

**Access**:
- Swagger UI: `http://localhost:8080/swagger-ui`
- OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`

**Implementation Details**:
- Uses external Swagger UI CDN for simplicity
- Serves OpenAPI specification from `backend/openapi.json`
- Includes all API endpoints with proper documentation
- Supports request/response examples

**API Endpoints Documented**:
- `GET /api/v1/health` - Health check
- `GET /api/v1/orders/orders` - Get all orders
- `POST /api/v1/orders/orders` - Create new order
- `GET /api/v1/orders/orders/{id}` - Get specific order
- `PUT /api/v1/orders/orders/{id}/cancel` - Cancel order
- `GET /api/v1/orders/orders/{id}/trades` - Get order trades

## Common Development Issues

### Port Conflicts

**Problem**: Services fail to start due to port conflicts.

**Solution**:
```bash
# Check what's running on a port
lsof -i :8080
lsof -i :5173

# Kill processes using specific ports
pkill -f "cargo run"
pkill -f "npm run dev"
```

### Docker Issues

**Problem**: Docker containers fail to start or run out of space.

**Solution**:
```bash
# Clean up Docker resources
docker system prune -f

# Check Docker disk usage
docker system df

# Remove unused containers and images
docker container prune -f
docker image prune -f
```

### Dependency Issues

**Problem**: Missing or incompatible dependencies.

**Solution**:
```bash
# Backend (Rust)
cargo clean
cargo update
cargo check

# Frontend (Node.js)
rm -rf node_modules package-lock.json
npm install
```

## Performance Issues

### Slow Build Times

**Problem**: Build times are too slow.

**Solution**:
```bash
# Backend optimization
cargo build --release

# Frontend optimization
npm run build
```

### Memory Issues

**Problem**: Services consume too much memory.

**Solution**:
- Monitor memory usage with `htop` or `top`
- Restart services periodically
- Check for memory leaks in application code

## Network Issues

### CORS Errors

**Problem**: Frontend can't communicate with backend due to CORS.

**Solution**:
- Backend is configured with CORS support
- Ensure frontend is making requests to the correct backend URL
- Check that the backend CORS configuration allows the frontend origin

### Connection Refused

**Problem**: Services can't connect to each other.

**Solution**:
- Verify that services are running on the correct ports
- Check firewall settings
- Ensure services are binding to the correct interfaces

## Testing Issues

### Test Failures

**Problem**: Tests are failing.

**Solution**:
```bash
# Backend tests
cargo test --no-default-features

# Frontend tests
npm test

# End-to-end tests
npm run test:e2e
```

## Deployment Issues

### Production Build Issues

**Problem**: Production builds fail or don't work correctly.

**Solution**:
```bash
# Backend production build
cargo build --release

# Frontend production build
npm run build
npm run preview
```

### Environment Variables

**Problem**: Missing or incorrect environment variables.

**Solution**:
- Check that all required environment variables are set
- Verify the format of environment variables
- Ensure sensitive data is properly secured

## Monitoring and Logging

### Log Analysis

**Problem**: Need to debug issues in production.

**Solution**:
```bash
# Check application logs
tail -f backend.log
tail -f frontend.log

# Check system logs
journalctl -u your-service-name
```

### Health Checks

**Problem**: Need to monitor service health.

**Solution**:
- Backend health check: `GET /api/v1/health`
- Frontend health check: Check if the application loads
- Database health check: Verify database connectivity

## Getting Help

If you encounter issues not covered in this guide:

1. Check the application logs for error messages
2. Review the browser console for frontend errors
3. Verify that all dependencies are up to date
4. Test with a clean environment
5. Check the project documentation
6. Review the API documentation at `/swagger-ui`

## Quick Commands

```bash
# Start backend (mock mode)
cargo run --no-default-features

# Start frontend
npm run dev

# Check if services are running
curl http://localhost:8080/api/v1/health
curl http://localhost:5173

# Access Swagger UI
open http://localhost:8080/swagger-ui
```
