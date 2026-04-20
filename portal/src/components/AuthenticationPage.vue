<script setup lang="ts">
import { ref, reactive } from 'vue';
import { useAuthStore } from '@/stores/useAuthStore';
import { AuthService } from '@/services/authService';

const auth = useAuthStore();
const isLogin = ref(true);
const error = ref('');
const form = reactive({ username: '', password: '' });

async function onSubmit() {
  error.value = '';
  
  if (!form.username || !form.password) {
    error.value = '请填写用户名和密码';
    return;
  }
  
  try {
    if (isLogin.value) {
      await auth.handleLogin(form);
    } else {
      await AuthService.register(form);
      error.value = '注册成功，请登录';
      isLogin.value = true;
      form.password = '';
    }
  } catch (e: any) {
    if (e.response?.status === 409) {
      error.value = '用户名已存在';
    } else if (e.response?.status === 401) {
      error.value = '用户名或密码错误';
    } else {
      error.value = '身份验证失败: ' + (e.response?.data?.message || e.message || '服务器错误');
    }
  }
}

function toggleMode() {
  isLogin.value = !isLogin.value;
  error.value = '';
  form.password = '';
}
</script>

<template>
  <div class="auth-wrapper">
    <form @submit.prevent="onSubmit" class="auth-form">
      <h2>{{ isLogin ? '登录' : '注册' }}</h2>
      
      <div v-if="error" class="error-message">
        {{ error }}
      </div>
      
      <div class="form-group">
        <label for="username">用户名</label>
        <input 
          id="username"
          v-model="form.username" 
          type="text" 
          placeholder="请输入用户名"
          required
        />
      </div>
      
      <div class="form-group">
        <label for="password">密码</label>
        <input 
          id="password"
          v-model="form.password" 
          type="password" 
          placeholder="请输入密码"
          required
        />
      </div>
      
      <button type="submit" class="submit-btn">
        {{ isLogin ? '进入系统' : '创建账户' }}
      </button>
      
      <button type="button" class="toggle-btn" @click="toggleMode">
        {{ isLogin ? '没有账户？去注册' : '已有账户？去登录' }}
      </button>
    </form>
  </div>
</template>

<style scoped>
.auth-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: #f1f5f9;
}

.auth-form {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  width: 100%;
  max-width: 400px;
  border-top: 4px solid #42b883;
}

h2 {
  text-align: center;
  margin-bottom: 1.5rem;
  color: #2c3e50;
  font-weight: 600;
}

.form-group {
  margin-bottom: 1.25rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  color: #476582;
  font-weight: 500;
  font-size: 0.9rem;
}

input {
  width: 100%;
  padding: 0.875rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  box-sizing: border-box;
  transition: all 0.2s;
  background: #fafafa;
}

input:focus {
  outline: none;
  border-color: #42b883;
  background: white;
  box-shadow: 0 0 0 3px rgba(66, 184, 131, 0.1);
}

.error-message {
  background: #fff5f5;
  color: #c53030;
  padding: 0.875rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  font-size: 0.875rem;
  border-left: 3px solid #fc8181;
}

.submit-btn {
  width: 100%;
  padding: 0.875rem;
  background: #42b883;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  margin-bottom: 0.75rem;
  transition: all 0.2s;
}

.submit-btn:hover {
  background: #3aa876;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(66, 184, 131, 0.3);
}

.toggle-btn {
  width: 100%;
  padding: 0.875rem;
  background: transparent;
  color: #42b883;
  border: 1px solid #42b883;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-btn:hover {
  background: rgba(66, 184, 131, 0.1);
}
</style>
