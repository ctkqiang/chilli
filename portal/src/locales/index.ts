import { createI18n } from 'vue-i18n';
import en from './en.json';
import zh from './zh.json';

// Get saved locale from localStorage or default to 'zh'
const savedLocale = localStorage.getItem('locale') || 'zh';

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'en',
  globalInjection: true,
  messages: {
    en,
    zh,
  },
});

export default i18n;
