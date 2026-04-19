import { defineStore } from 'pinia';
import { AuthService } from '@/services/authService';
import type { UserCredentials } from '@/models/auth';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: localStorage.getItem('chilli_token') || '',
    currentUser: localStorage.getItem('chilli_user') || ''
  }),
  actions: {
    async handleLogin(creds: UserCredentials) {
      const data = await AuthService.login(creds);
      this.token = data.token;
      this.currentUser = creds.username;
      localStorage.setItem('chilli_token', data.token);
      localStorage.setItem('chilli_user', creds.username);
    },
    logout() {
      this.token = '';
      localStorage.removeItem('chilli_token');
    }
  }
});