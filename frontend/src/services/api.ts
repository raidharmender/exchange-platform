import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor to handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Types
export interface CreateOrderRequest {
  symbol: string;
  side: 'Buy' | 'Sell';
  quantity: string;
  price: string;
  order_type: 'Market' | 'Limit' | 'Stop' | 'StopLimit';
}

export interface Order {
  id: string;
  symbol: string;
  side: 'Buy' | 'Sell';
  quantity: string;
  price: string;
  order_type: 'Market' | 'Limit' | 'Stop' | 'StopLimit';
  status: 'New' | 'Open' | 'PartiallyFilled' | 'Filled' | 'Cancelled' | 'Rejected';
  filled_quantity: string;
  created_at: string;
}

export interface OrderQuery {
  symbol?: string;
  status?: string;
  limit?: number;
  offset?: number;
}

// API endpoints
export const authAPI = {
  login: (email: string, password: string) =>
    api.post('/api/v1/auth/login', { email, password }),
  register: (email: string, password: string) =>
    api.post('/api/v1/auth/register', { email, password }),
  me: () => api.get('/api/v1/users/me'),
};

export const ordersAPI = {
  getOrders: (params?: OrderQuery) => api.get<Order[]>('/api/v1/orders/orders', { params }),
  getOrder: (id: string) => api.get<Order>(`/api/v1/orders/orders/${id}`),
  createOrder: (order: CreateOrderRequest) => api.post<Order>('/api/v1/orders/orders', order),
  cancelOrder: (id: string) => api.put<Order>(`/api/v1/orders/orders/${id}/cancel`),
  getOrderTrades: (id: string) => api.get(`/api/v1/orders/orders/${id}/trades`),
};

export const tradesAPI = {
  getTrades: (params?: any) => api.get('/api/v1/trades', { params }),
  getTrade: (id: string) => api.get(`/api/v1/trades/${id}`),
};

export const orderBookAPI = {
  getOrderBook: (symbol: string) => api.get(`/api/v1/orderbook/${symbol}`),
};

export const marketDataAPI = {
  getMarketData: (symbol: string) => api.get(`/api/v1/market-data/${symbol}`),
  getMarketDataAll: () => api.get('/api/v1/market-data'),
}; 