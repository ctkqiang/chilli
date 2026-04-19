import axios from 'axios';
import { config } from 'dotenv';
import { resolve } from 'path';

const envPath = resolve(__dirname, '../../../.env');
config({ path: envPath });

const api = axios.create({
  baseURL: process.env.CORE_PORT
    ? `http://localhost:${process.env.CORE_PORT}/api`
    : 'http://localhost:9333/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

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

export default api;
