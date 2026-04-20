import { createI18n } from 'vue-i18n';
import en from './en.json';
import zh from './zh.json';

const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages: {
    en,
    zh,
  },
});

export default i18n;
