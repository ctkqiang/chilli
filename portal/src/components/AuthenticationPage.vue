<script setup lang="ts">
import { ref, reactive } from 'vue';
import { useAuthStore } from '@/stores/useAuthStore';
import { AuthService } from '@/services/authService';

const auth = useAuthStore();
const isLogin = ref(true);
const error = ref('');
const form = reactive({ username: '', password: '' });

async function onSubmit() {
  try {
    if (isLogin.value) {
      await auth.handleLogin(form);
    } else {
      await AuthService.register(form);
      isLogin.value = true;
    }
  } catch (e: any) {
    error.value = "身份验证失败: " + (e.response?.data || "服务器断开");
  }
}
</script>

<template>
  <div class="auth-wrapper">
    <form @submit.prevent="onSubmit">
       <input v-model="form.username" />
       <input v-model="form.password" />
       <button type="submit">{{ isLogin ? '进入系统' : '创建账户' }}</button>
    </form>
  </div>
</template>