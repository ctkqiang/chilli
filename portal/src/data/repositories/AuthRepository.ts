import api from '@/api/axios';
import type { UserCredentials, AuthResponse } from '@/models/auth';

export class AuthRepository {
  async login(credentials: UserCredentials): Promise<AuthResponse> {
    const { data } = await api.post<AuthResponse>('/auth/login', credentials);
    return data;
  }

  async register(credentials: UserCredentials): Promise<void> {
    await api.post('/auth/register', credentials);
  }
}

export const authRepository = new AuthRepository();
