import api from '@/api/axios';
import type { UserCredentials, AuthResponse } from '@/models/auth';

export const AuthService = {
  async login(creds: UserCredentials): Promise<AuthResponse> {
    const { data } = await api.post<AuthResponse>('/auth/login', creds);
    return data;
  },

  async register(creds: UserCredentials): Promise<void> {
    await api.post('/auth/register', creds);
  }
};