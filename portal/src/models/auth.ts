export interface UserCredentials {
  username: string;
  password?: string;
}

export interface AuthResponse {
  token: string;
}

export interface UserState {
  username: string;
  isLoggedIn: boolean;
}