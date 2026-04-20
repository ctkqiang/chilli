import { defineStore } from 'pinia';
import { ref, watch, onMounted } from 'vue';

type Theme = 'light' | 'dark' | 'auto';

export const useThemeStore = defineStore('theme', () => {
  const theme = ref<Theme>('auto');
  const isDark = ref(false);

  const systemPrefersDark = () => {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  };

  const updateTheme = () => {
    if (theme.value === 'auto') {
      isDark.value = systemPrefersDark();
    } else {
      isDark.value = theme.value === 'dark';
    }

    // Apply theme to document
    if (isDark.value) {
      document.documentElement.classList.add('dark');
      document.documentElement.setAttribute('data-theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      document.documentElement.setAttribute('data-theme', 'light');
    }
  };

  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme;
    localStorage.setItem('theme', newTheme);
    updateTheme();
  };

  const toggleTheme = () => {
    const newTheme = isDark.value ? 'light' : 'dark';
    setTheme(newTheme);
  };

  // Listen for system theme changes
  onMounted(() => {
    const savedTheme = localStorage.getItem('theme') as Theme | null;
    if (savedTheme) {
      theme.value = savedTheme;
    }
    updateTheme();

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', () => {
      if (theme.value === 'auto') {
        updateTheme();
      }
    });
  });

  watch(theme, updateTheme);

  return {
    theme,
    isDark,
    setTheme,
    toggleTheme,
  };
});
