# Exchange Platform - Operational Runbook & Troubleshooting Guide

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Development Setup](#development-setup)
3. [Deployment Procedures](#deployment-procedures)
4. [Testing Procedures](#testing-procedures)
5. [Monitoring and Alerting](#monitoring-and-alerting)
6. [Troubleshooting](#troubleshooting)
7. [Maintenance Procedures](#maintenance-procedures)
8. [Emergency Procedures](#emergency-procedures)
9. [Security Procedures](#security-procedures)

## Prerequisites

### Required Tools
- `kubectl` (v1.24+)
- `helm` (v3.8+)
- `docker` (v20.10+)
- `minikube` or access to a Kubernetes cluster
- `git`
- `rust` (v1.70+) - for backend development
- `node.js` (v18+) - for frontend development
- `npm` or `yarn` - for frontend dependencies

### Required Access
- Kubernetes cluster access
- Docker registry access
- Database access (PostgreSQL)
- Monitoring system access (Prometheus/Grafana)

## Development Setup

### 1. Backend Setup

#### 1.1 Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install PostgreSQL (optional for development)
# For macOS: brew install postgresql
# For Ubuntu: sudo apt-get install postgresql
```

#### 1.2 Environment Configuration
```bash
# Navigate to backend directory
cd backend

# Create .env file
cat > .env << EOF
DATABASE_URL=postgresql://user:password@localhost:5432/exchange
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-here-change-in-production
RUST_LOG=info
EOF
```

#### 1.3 Running Backend (Development Mode)
```bash
# Run without database (mock mode)
cargo run --no-default-features

# Run with database (requires PostgreSQL and Redis)
cargo run --features database
```

#### 1.4 Backend Testing
```bash
# Run all tests
cargo test

# Run tests without database
cargo test --no-default-features

# Run tests with output
cargo test -- --nocapture
```

### 2. Frontend Setup

#### 2.1 Prerequisites
```bash
# Install Node.js (if not already installed)
# For macOS: brew install node
# For Ubuntu: sudo apt-get install nodejs npm
```

#### 2.2 Environment Configuration
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install
```

#### 2.3 Running Frontend (Development Mode)
```bash
# Start development server
npm run dev

# The frontend will be available at http://localhost:5173
```

#### 2.4 Frontend Testing
```bash
# Run unit tests
npm test

# Run tests in watch mode
npm run test:ui

# Run end-to-end tests (requires Playwright)
npm run test:e2e
```

### 3. Full Stack Development

#### 3.1 Starting Both Services
```bash
# Terminal 1 - Backend
cd backend
cargo run --no-default-features

# Terminal 2 - Frontend
cd frontend
npm run dev
```

#### 3.2 Health Checks
```bash
# Test backend health
curl http://localhost:8080/api/v1/health

# Test frontend
curl http://localhost:5173
```

## Testing Procedures

### 1. Backend Tests

#### 1.1 Unit Tests
```bash
# Run all unit tests
cargo test

# Run specific test module
cargo test models::tests

# Run tests with verbose output
cargo test -- --nocapture
```

#### 1.2 Integration Tests
```bash
# Run integration tests (requires database)
cargo test --features database

# Run tests with database connection
DATABASE_URL=postgresql://user:password@localhost:5432/exchange cargo test
```

#### 1.3 API Tests
```bash
# Test health endpoint
curl -X GET http://localhost:8080/api/v1/health

# Test order creation
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "1.0",
    "price": "50000.00",
    "order_type": "Limit"
  }'
```

### 2. Frontend Tests

#### 2.1 Unit Tests
```bash
# Run all unit tests
npm test

# Run tests in watch mode
npm run test:ui

# Run tests with coverage
npm run test -- --coverage
```

#### 2.2 Component Tests
```bash
# Test specific component
npm test -- App.test.tsx

# Test with specific pattern
npm test -- --grep "App"
```

#### 2.3 End-to-End Tests
```bash
# Install Playwright browsers (first time only)
npx playwright install

# Run E2E tests
npm run test:e2e

# Run E2E tests in headed mode
npx playwright test --headed
```

### 3. Integration Tests

#### 3.1 Full Stack Testing
```bash
# Start both services
# Terminal 1
cd backend && cargo run --no-default-features

# Terminal 2
cd frontend && npm run dev

# Terminal 3 - Run integration tests
npm run test:integration
```

#### 3.2 API Integration Tests
```bash
# Test complete order flow
curl -X POST http://localhost:8080/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC/USD",
    "side": "Buy",
    "quantity": "1.0",
    "price": "50000.00",
    "order_type": "Limit"
  }'

# Get order details
curl -X GET http://localhost:8080/api/v1/orders/{order_id}
```

## Deployment Procedures

### 1. Initial Setup

#### 1.1 Create Namespace
```bash
kubectl apply -f kubernetes/namespace.yml
```

#### 1.2 Apply Secrets and ConfigMaps
```bash
# Apply secrets (update with actual values)
kubectl apply -f kubernetes/secrets.yml

# Apply configuration
kubectl apply -f kubernetes/configmap.yml
```

#### 1.3 Deploy Infrastructure
```bash
# Deploy PostgreSQL
kubectl apply -f kubernetes/postgres-deployment.yml

# Deploy Redis
kubectl apply -f kubernetes/redis-deployment.yml

# Wait for infrastructure to be ready
kubectl wait --for=condition=ready pod -l app=postgres -n exchange --timeout=300s
kubectl wait --for=condition=ready pod -l app=redis -n exchange --timeout=300s
```

#### 1.4 Deploy Applications
```bash
# Deploy API
kubectl apply -f kubernetes/api-deployment.yml

# Deploy Frontend
kubectl apply -f kubernetes/frontend-deployment.yml

# Deploy Ingress
kubectl apply -f kubernetes/ingress.yml
```

### 2. Verification

#### 2.1 Check Pod Status
```bash
kubectl get pods -n exchange
```

#### 2.2 Check Services
```bash
kubectl get svc -n exchange
```

#### 2.3 Check Ingress
```bash
kubectl get ingress -n exchange
```

#### 2.4 Test Endpoints
```bash
# Test API health
curl http://exchange.local/api/v1/health

# Test frontend
curl http://exchange.local
```

## Monitoring and Alerting

### 1. Metrics Collection

#### 1.1 Prometheus Configuration
```yaml
# prometheus-config.yml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
  namespace: exchange
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
    scrape_configs:
    - job_name: 'exchange-api'
      static_configs:
      - targets: ['exchange-api:8080']
      metrics_path: /metrics
```

#### 1.2 Key Metrics to Monitor
- **API Metrics**
  - Request rate (RPS)
  - Response time (latency)
  - Error rate
  - Active connections

- **Order Book Metrics**
  - Order processing time
  - Trade execution rate
  - Order book depth
  - Matching engine performance

- **Database Metrics**
  - Connection pool usage
  - Query performance
  - Transaction rate
  - Storage usage

- **Infrastructure Metrics**
  - CPU usage
  - Memory usage
  - Disk I/O
  - Network traffic

### 2. Alerting Rules

#### 2.1 Critical Alerts
```yaml
# critical-alerts.yml
groups:
- name: exchange-critical
  rules:
  - alert: APIHighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 2m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value }} errors per second"

  - alert: OrderBookDown
    expr: up{job="exchange-api"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Order book service is down"
```

#### 2.2 Warning Alerts
```yaml
# warning-alerts.yml
groups:
- name: exchange-warnings
  rules:
  - alert: HighLatency
    expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 0.5
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High latency detected"
```

## Troubleshooting

### 1. Frontend Issues

#### 1.1 Blank Frontend Page

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

### 2. Backend Issues

#### 2.1 Database Connection Issues

**Problem**: Backend fails to start due to database connection errors.

**Solution**:
```bash
# Run in mock mode (no database required)
cargo run --no-default-features

# Or run with database (requires PostgreSQL and Redis)
cargo run --features database
```

#### 2.2 Swagger UI Implementation

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

### 3. Common Development Issues

#### 3.1 Port Conflicts

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

#### 3.2 Docker Issues

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

#### 3.3 Dependency Issues

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

### 4. Performance Issues

#### 4.1 Slow Build Times

**Problem**: Build times are too slow.

**Solution**:
```bash
# Backend optimization
cargo build --release

# Frontend optimization
npm run build
```

#### 4.2 Memory Issues

**Problem**: Services consume too much memory.

**Solution**:
- Monitor memory usage with `htop` or `top`
- Restart services periodically
- Check for memory leaks in application code

### 5. Network Issues

#### 5.1 CORS Errors

**Problem**: Frontend can't communicate with backend due to CORS.

**Solution**:
- Backend is configured with CORS support
- Ensure frontend is making requests to the correct backend URL
- Check that the backend CORS configuration allows the frontend origin

#### 5.2 Connection Refused

**Problem**: Services can't connect to each other.

**Solution**:
- Verify that services are running on the correct ports
- Check firewall settings
- Ensure services are binding to the correct interfaces

### 6. Testing Issues

#### 6.1 Test Failures

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

### 7. Deployment Issues

#### 7.1 Production Build Issues

**Problem**: Production builds fail or don't work correctly.

**Solution**:
```bash
# Backend production build
cargo build --release

# Frontend production build
npm run build
npm run preview
```

#### 7.2 Environment Variables

**Problem**: Missing or incorrect environment variables.

**Solution**:
- Check that all required environment variables are set
- Verify the format of environment variables
- Ensure sensitive data is properly secured

### 8. Monitoring and Logging

#### 8.1 Log Analysis

**Problem**: Need to debug issues in production.

**Solution**:
```bash
# Check application logs
tail -f backend.log
tail -f frontend.log

# Check system logs
journalctl -u your-service-name
```

#### 8.2 Health Checks

**Problem**: Need to monitor service health.

**Solution**:
- Backend health check: `GET /api/v1/health`
- Frontend health check: Check if the application loads
- Database health check: Verify database connectivity

### 9. Common Issues

#### 9.1 API Service Not Responding
```bash
# Check pod status
kubectl get pods -l app=exchange-api -n exchange

# Check logs
kubectl logs -l app=exchange-api -n exchange

# Check service endpoints
kubectl get endpoints exchange-api -n exchange

# Check ingress
kubectl describe ingress exchange-ingress -n exchange
```

#### 9.2 Database Connection Issues
```bash
# Check PostgreSQL pod
kubectl get pods -l app=postgres -n exchange

# Check PostgreSQL logs
kubectl logs -l app=postgres -n exchange

# Test database connection
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange -c "SELECT 1;"
```

#### 9.3 Frontend Issues
```bash
# Check frontend pod
kubectl get pods -l app=exchange-frontend -n exchange

# Check frontend logs
kubectl logs -l app=exchange-frontend -n exchange

# Check frontend service
kubectl get svc exchange-frontend -n exchange
```

### 10. Performance Issues

#### 10.1 High CPU Usage
```bash
# Check resource usage
kubectl top pods -n exchange

# Check resource limits
kubectl describe pod -l app=exchange-api -n exchange

# Scale up if needed
kubectl scale deployment exchange-api --replicas=5 -n exchange
```

#### 10.2 High Memory Usage
```bash
# Check memory usage
kubectl top pods -n exchange

# Check memory limits
kubectl describe pod -l app=exchange-api -n exchange

# Check for memory leaks
kubectl logs -l app=exchange-api -n exchange | grep -i "out of memory"
```

### 11. Network Issues

#### 11.1 Service Communication
```bash
# Test service connectivity
kubectl exec -it deployment/exchange-api -n exchange -- curl -v http://postgres:5432

# Check network policies
kubectl get networkpolicies -n exchange

# Check DNS resolution
kubectl exec -it deployment/exchange-api -n exchange -- nslookup postgres
```

## Maintenance Procedures

### 1. Regular Maintenance

#### 1.1 Database Maintenance
```bash
# Backup database
kubectl exec -it deployment/postgres -n exchange -- pg_dump -U user exchange > backup.sql

# Vacuum database
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange -c "VACUUM ANALYZE;"

# Check database size
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange -c "SELECT pg_size_pretty(pg_database_size('exchange'));"
```

#### 1.2 Log Rotation
```bash
# Check log sizes
kubectl exec -it deployment/exchange-api -n exchange -- du -sh /var/log

# Rotate logs if needed
kubectl exec -it deployment/exchange-api -n exchange -- logrotate -f /etc/logrotate.conf
```

#### 1.3 Certificate Renewal
```bash
# Check certificate expiration
kubectl get secrets -n exchange -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.data.tls\.crt}{"\n"}{end}' | while read name cert; do echo "$name: $(echo $cert | base64 -d | openssl x509 -noout -enddate)"; done

# Renew certificates if needed
# (Follow your certificate management process)
```

### 2. Updates and Upgrades

#### 2.1 Application Updates
```bash
# Update API image
kubectl set image deployment/exchange-api exchange-api=exchange/api:v1.1.0 -n exchange

# Update frontend image
kubectl set image deployment/exchange-frontend exchange-frontend=exchange/frontend:v1.1.0 -n exchange

# Monitor rollout
kubectl rollout status deployment/exchange-api -n exchange
kubectl rollout status deployment/exchange-frontend -n exchange
```

#### 2.2 Database Migrations
```bash
# Run migrations
kubectl exec -it deployment/exchange-api -n exchange -- sqlx migrate run

# Check migration status
kubectl exec -it deployment/exchange-api -n exchange -- sqlx migrate info
```

## Emergency Procedures

### 1. Service Outage

#### 1.1 Immediate Response
1. **Assess Impact**
   - Check service status
   - Identify affected users
   - Estimate downtime

2. **Communicate**
   - Notify stakeholders
   - Update status page
   - Send alerts to team

3. **Investigate**
   - Check logs
   - Review recent changes
   - Identify root cause

#### 1.2 Recovery Steps
```bash
# Restart services if needed
kubectl rollout restart deployment/exchange-api -n exchange
kubectl rollout restart deployment/exchange-frontend -n exchange

# Check recovery
kubectl rollout status deployment/exchange-api -n exchange
kubectl rollout status deployment/exchange-frontend -n exchange
```

### 2. Data Loss

#### 2.1 Immediate Response
1. **Stop Data Loss**
   - Identify source of data loss
   - Stop affected services
   - Prevent further damage

2. **Assess Damage**
   - Determine scope of data loss
   - Identify affected data
   - Estimate recovery time

3. **Recovery Plan**
   - Restore from backup
   - Replay transactions
   - Verify data integrity

#### 2.2 Recovery Steps
```bash
# Restore database from backup
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange < backup.sql

# Verify data integrity
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange -c "SELECT COUNT(*) FROM orders;"
kubectl exec -it deployment/postgres -n exchange -- psql -U user -d exchange -c "SELECT COUNT(*) FROM trades;"
```

## Security Procedures

### 1. Security Monitoring

#### 1.1 Access Monitoring
```bash
# Check for unauthorized access
kubectl logs -l app=exchange-api -n exchange | grep -i "unauthorized\|forbidden"

# Check authentication logs
kubectl logs -l app=exchange-api -n exchange | grep -i "login\|auth"
```

#### 1.2 Vulnerability Scanning
```bash
# Scan for vulnerabilities
kubectl run security-scan --image=trivy/trivy --rm -it -- scan exchange/api:latest

# Check for outdated dependencies
kubectl exec -it deployment/exchange-api -n exchange -- cargo audit
```

### 2. Incident Response

#### 2.1 Security Incident
1. **Immediate Response**
   - Isolate affected systems
   - Preserve evidence
   - Notify security team

2. **Investigation**
   - Analyze logs
   - Identify attack vector
   - Assess damage

3. **Recovery**
   - Patch vulnerabilities
   - Restore systems
   - Update security measures

#### 2.2 Post-Incident
1. **Documentation**
   - Document incident details
   - Update procedures
   - Train team members

2. **Prevention**
   - Implement additional security measures
   - Update monitoring
   - Review access controls

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

## Contact Information

### Emergency Contacts
- **On-Call Engineer**: [Phone Number]
- **System Administrator**: [Email]
- **Security Team**: [Email]
- **Management**: [Email]

### Escalation Matrix
1. **Level 1**: On-call engineer (0-30 minutes)
2. **Level 2**: System administrator (30-60 minutes)
3. **Level 3**: Security team (60-120 minutes)
4. **Level 4**: Management (120+ minutes)

## Appendix

### A. Useful Commands
```bash
# Get all resources in namespace
kubectl get all -n exchange

# Describe resource
kubectl describe pod <pod-name> -n exchange

# Port forward for debugging
kubectl port-forward svc/exchange-api 8080:8080 -n exchange

# Execute command in pod
kubectl exec -it <pod-name> -n exchange -- /bin/bash

# View logs with follow
kubectl logs -f <pod-name> -n exchange
```

### B. Configuration Files
- [Kubernetes manifests](kubernetes/)
- [Helm charts](helm/)
- [Docker files](docker/)

### C. Documentation
- [Architecture Document](ARCHITECTURE.md)
- [API Documentation](docs/API.md)
- [Development Guide](docs/DEVELOPMENT.md) 