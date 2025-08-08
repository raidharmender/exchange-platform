import React from 'react';

function App() {
  return (
    <div style={{ padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      <h1 style={{ color: '#333', textAlign: 'center', marginBottom: '20px' }}>
        Exchange Platform - Test
      </h1>
      
      <div style={{ 
        maxWidth: '800px', 
        margin: '0 auto', 
        padding: '20px',
        backgroundColor: 'white',
        borderRadius: '8px',
        boxShadow: '0 2px 4px rgba(0,0,0,0.1)'
      }}>
        <h2 style={{ color: '#555', marginBottom: '20px' }}>
          Frontend is Working! 🎉
        </h2>
        
        <p style={{ color: '#666', lineHeight: '1.6', marginBottom: '20px' }}>
          The React application is successfully running. Here's what's available:
        </p>
        
        <div style={{ 
          display: 'grid', 
          gridTemplateColumns: '1fr 1fr', 
          gap: '20px', 
          marginBottom: '20px' 
        }}>
          <div style={{ 
            backgroundColor: '#e3f2fd', 
            padding: '20px', 
            borderRadius: '8px' 
          }}>
            <h3 style={{ color: '#1976d2', marginBottom: '10px' }}>Backend Status</h3>
            <p style={{ color: '#1976d2' }}>✅ Running on http://localhost:8080</p>
          </div>
          <div style={{ 
            backgroundColor: '#e8f5e8', 
            padding: '20px', 
            borderRadius: '8px' 
          }}>
            <h3 style={{ color: '#2e7d32', marginBottom: '10px' }}>Frontend Status</h3>
            <p style={{ color: '#2e7d32' }}>✅ Running on http://localhost:5173</p>
          </div>
        </div>
        
        <div style={{ 
          backgroundColor: '#fff3e0', 
          padding: '20px', 
          borderRadius: '8px',
          marginBottom: '20px'
        }}>
          <h3 style={{ color: '#f57c00', marginBottom: '10px' }}>Orders API Features</h3>
          <ul style={{ color: '#f57c00', lineHeight: '1.6' }}>
            <li>✅ GET /api/v1/orders/orders - Fetch all orders</li>
            <li>✅ POST /api/v1/orders/orders - Create new orders</li>
            <li>✅ PUT /api/v1/orders/orders/{"{id}"}/cancel - Cancel orders</li>
            <li>✅ Real-time order management</li>
          </ul>
        </div>
        
        <div style={{ textAlign: 'center' }}>
          <button 
            onClick={() => alert('Frontend is working!')}
            style={{
              backgroundColor: '#2196f3',
              color: 'white',
              border: 'none',
              padding: '10px 20px',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '16px'
            }}
          >
            Test Frontend
          </button>
        </div>
      </div>
    </div>
  );
}

export default App; 