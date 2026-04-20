<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import i18n from '@/locales';
import { useThemeStore } from '@/stores/themeStore';

const route = useRoute();
const { t } = useI18n();
const themeStore = useThemeStore();

// Get global locale from i18n instance
type Locale = 'en' | 'zh';
const locale = computed({
  get: () => i18n.global.locale.value as Locale,
  set: (val: Locale) => { i18n.global.locale.value = val; }
});

const scrolled = ref(false);
const mobileMenuOpen = ref(false);
const langDropdownOpen = ref(false);
const themeDropdownOpen = ref(false);

const handleScroll = () => {
  scrolled.value = window.scrollY > 20;
};

onMounted(() => {
  window.addEventListener('scroll', handleScroll, { passive: true });
});

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll);
});

const navLinks = [
  { path: '/', name: 'nav.dashboard', icon: '📊' },
  { path: '/processes', name: 'nav.processes', icon: '⚡' },
  { path: '/security', name: 'nav.security', icon: '🛡️' },
  { path: '/docker', name: 'nav.docker', icon: '🐳' },
];

const isActive = (path: string) => route.path === path;

const languages = [
  { code: 'en', name: 'common.english', flag: '🇺🇸' },
  { code: 'zh', name: 'common.chinese', flag: '🇨🇳' },
];

const themes = [
  { code: 'light', name: 'common.light', icon: '☀️' },
  { code: 'dark', name: 'common.dark', icon: '🌙' },
  { code: 'auto', name: 'common.auto', icon: '⚡' },
];

const currentLanguage = computed(() => {
  return languages.find(lang => lang.code === locale.value) || languages[0];
});

const currentTheme = computed(() => {
  return themes.find(t => t.code === themeStore.theme) || themes[2];
});

const setLanguage = (code: string) => {
  locale.value = code as Locale;
  localStorage.setItem('locale', code);
  langDropdownOpen.value = false;
};

const setTheme = (code: string) => {
  themeStore.setTheme(code as 'light' | 'dark' | 'auto');
  themeDropdownOpen.value = false;
};

// Close dropdowns when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  if (!target.closest('.dropdown')) {
    langDropdownOpen.value = false;
    themeDropdownOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<template>
  <div class="app">
    <!-- Animated Background -->
    <div class="bg-animation">
      <div class="bg-gradient"></div>
      <div class="bg-grid"></div>
    </div>

    <!-- Navigation -->
    <nav class="navbar" :class="{ 'scrolled': scrolled }">
      <div class="nav-container">
        <div class="nav-brand">
          <div class="logo-wrapper">
            <span class="logo">🌶️</span>
            <div class="logo-glow"></div>
          </div>
          <div class="brand-text">
            <span class="brand-name">{{ t('app.name') }}</span>
            <span class="brand-tagline">{{ t('app.tagline') }}</span>
          </div>
        </div>

        <!-- Desktop Navigation -->
        <div class="nav-links">
          <router-link
            v-for="link in navLinks"
            :key="link.path"
            :to="link.path"
            class="nav-link"
            :class="{ 'active': isActive(link.path) }"
          >
            <span class="nav-icon">{{ link.icon }}</span>
            <span class="nav-text">{{ t(link.name) }}</span>
            <div class="nav-indicator"></div>
          </router-link>
        </div>

        <!-- Controls -->
        <div class="nav-controls">
          <!-- Language Dropdown -->
          <div class="dropdown">
            <button 
              class="control-btn" 
              @click.stop="langDropdownOpen = !langDropdownOpen"
              :class="{ 'active': langDropdownOpen }"
            >
              <span class="control-icon">{{ currentLanguage.flag }}</span>
              <span class="control-text">{{ t(currentLanguage.name) }}</span>
              <span class="dropdown-arrow" :class="{ 'open': langDropdownOpen }">▼</span>
            </button>
            <div class="dropdown-menu" :class="{ 'open': langDropdownOpen }">
              <button
                v-for="lang in languages"
                :key="lang.code"
                class="dropdown-item"
                :class="{ 'active': locale === lang.code }"
                @click="setLanguage(lang.code)"
              >
                <span class="item-icon">{{ lang.flag }}</span>
                <span class="item-text">{{ t(lang.name) }}</span>
              </button>
            </div>
          </div>

          <!-- Theme Dropdown -->
          <div class="dropdown">
            <button 
              class="control-btn" 
              @click.stop="themeDropdownOpen = !themeDropdownOpen"
              :class="{ 'active': themeDropdownOpen }"
            >
              <span class="control-icon">{{ currentTheme.icon }}</span>
              <span class="control-text">{{ t(currentTheme.name) }}</span>
              <span class="dropdown-arrow" :class="{ 'open': themeDropdownOpen }">▼</span>
            </button>
            <div class="dropdown-menu" :class="{ 'open': themeDropdownOpen }">
              <button
                v-for="theme in themes"
                :key="theme.code"
                class="dropdown-item"
                :class="{ 'active': themeStore.theme === theme.code }"
                @click="setTheme(theme.code)"
              >
                <span class="item-icon">{{ theme.icon }}</span>
                <span class="item-text">{{ t(theme.name) }}</span>
              </button>
            </div>
          </div>
        </div>

        <!-- Mobile Menu Button -->
        <button class="mobile-menu-btn" @click="mobileMenuOpen = !mobileMenuOpen">
          <span class="menu-line" :class="{ 'open': mobileMenuOpen }"></span>
          <span class="menu-line" :class="{ 'open': mobileMenuOpen }"></span>
          <span class="menu-line" :class="{ 'open': mobileMenuOpen }"></span>
        </button>
      </div>

      <!-- Mobile Menu -->
      <div class="mobile-menu" :class="{ 'open': mobileMenuOpen }">
        <router-link
          v-for="link in navLinks"
          :key="link.path"
          :to="link.path"
          class="mobile-nav-link"
          :class="{ 'active': isActive(link.path) }"
          @click="mobileMenuOpen = false"
        >
          <span class="nav-icon">{{ link.icon }}</span>
          <span class="nav-text">{{ t(link.name) }}</span>
        </router-link>
        
        <!-- Mobile Controls -->
        <div class="mobile-controls">
          <div class="mobile-control-group">
            <span class="mobile-control-label">{{ t('common.language') }}</span>
            <div class="mobile-control-options">
              <button
                v-for="lang in languages"
                :key="lang.code"
                class="mobile-option-btn"
                :class="{ 'active': locale === lang.code }"
                @click="setLanguage(lang.code)"
              >
                {{ lang.flag }} {{ t(lang.name) }}
              </button>
            </div>
          </div>
          <div class="mobile-control-group">
            <span class="mobile-control-label">{{ t('common.theme') }}</span>
            <div class="mobile-control-options">
              <button
                v-for="theme in themes"
                :key="theme.code"
                class="mobile-option-btn"
                :class="{ 'active': themeStore.theme === theme.code }"
                @click="setTheme(theme.code)"
              >
                {{ theme.icon }} {{ t(theme.name) }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <main class="main-content">
      <div class="content-wrapper">
        <RouterView v-slot="{ Component }">
          <transition name="page" mode="out-in">
            <component :is="Component" />
          </transition>
        </RouterView>
      </div>
    </main>

    <!-- Footer -->
    <footer class="footer">
      <div class="footer-content">
        <span class="footer-text">{{ t('app.footer') }}</span>
        <span class="footer-version">{{ t('app.version', { version: '1.0.0' }) }}</span>
      </div>
    </footer>
  </div>
</template>

<style scoped>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
}

/* Animated Background */
.bg-animation {
  position: fixed;
  inset: 0;
  z-index: -1;
  overflow: hidden;
}

.bg-gradient {
  position: absolute;
  inset: 0;
  background: 
    radial-gradient(ellipse at 20% 20%, rgba(99, 102, 241, 0.15) 0%, transparent 50%),
    radial-gradient(ellipse at 80% 80%, rgba(236, 72, 153, 0.1) 0%, transparent 50%),
    radial-gradient(ellipse at 50% 50%, rgba(6, 182, 212, 0.05) 0%, transparent 70%);
  animation: gradientMove 20s ease-in-out infinite;
}

@keyframes gradientMove {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(30px, -30px) scale(1.1);
  }
  66% {
    transform: translate(-20px, 20px) scale(0.9);
  }
}

.bg-grid {
  position: absolute;
  inset: 0;
  background-image: 
    linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
  animation: gridMove 30s linear infinite;
}

@keyframes gridMove {
  0% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(60px, 60px);
  }
}

/* Navigation */
.navbar {
  position: sticky;
  top: 0;
  z-index: var(--z-sticky);
  padding: 1rem 2rem;
  transition: all var(--transition-base);
}

.navbar.scrolled {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  padding: 0.75rem 2rem;
}

.dark .navbar.scrolled {
  background: rgba(15, 23, 42, 0.8);
  border-bottom-color: rgba(51, 65, 85, 0.6);
}

.nav-container {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

/* Brand */
.nav-brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-wrapper {
  position: relative;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo {
  font-size: 2rem;
  z-index: 1;
  animation: float 3s ease-in-out infinite;
}

.logo-glow {
  position: absolute;
  inset: 0;
  background: radial-gradient(circle, rgba(99, 102, 241, 0.4) 0%, transparent 70%);
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

.brand-text {
  display: flex;
  flex-direction: column;
}

.brand-name {
  font-size: 1.5rem;
  font-weight: 800;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.02em;
}

.brand-tagline {
  font-size: 0.75rem;
  color: var(--color-text-soft);
  text-transform: uppercase;
  letter-spacing: 0.1em;
}

/* Navigation Links */
.nav-links {
  display: flex;
  gap: 0.5rem;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  color: var(--color-text);
  text-decoration: none;
  font-weight: 500;
  font-size: 0.9375rem;
  border-radius: var(--radius-lg);
  position: relative;
  transition: all var(--transition-base);
  overflow: hidden;
}

.nav-link::before {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--gradient-primary);
  opacity: 0;
  transition: opacity var(--transition-base);
  border-radius: inherit;
}

.nav-link:hover {
  color: var(--color-primary);
  transform: translateY(-2px);
}

.nav-link.active {
  color: white;
}

.nav-link.active::before {
  opacity: 1;
}

.nav-icon,
.nav-text {
  position: relative;
  z-index: 1;
}

.nav-indicator {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%) scaleX(0);
  width: 20px;
  height: 3px;
  background: var(--gradient-primary);
  border-radius: var(--radius-full);
  transition: transform var(--transition-base);
}

.nav-link:hover .nav-indicator {
  transform: translateX(-50%) scaleX(1);
}

.nav-link.active .nav-indicator {
  display: none;
}

/* Nav Controls */
.nav-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* Dropdown */
.dropdown {
  position: relative;
}

.control-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1rem;
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  color: var(--color-text);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-base);
}

.control-btn:hover,
.control-btn.active {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.control-icon {
  font-size: 1rem;
}

.control-text {
  display: none;
}

@media (min-width: 1024px) {
  .control-text {
    display: inline;
  }
}

.dropdown-arrow {
  font-size: 0.625rem;
  transition: transform var(--transition-base);
}

.dropdown-arrow.open {
  transform: rotate(180deg);
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  min-width: 140px;
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  padding: 0.5rem;
  opacity: 0;
  visibility: hidden;
  transform: translateY(-10px);
  transition: all var(--transition-base);
  z-index: var(--z-dropdown);
}

.dropdown-menu.open {
  opacity: 1;
  visibility: visible;
  transform: translateY(0);
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.625rem 0.875rem;
  background: none;
  border: none;
  border-radius: var(--radius);
  color: var(--color-text);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all var(--transition-fast);
  text-align: left;
}

.dropdown-item:hover {
  background: var(--color-gray-100);
}

.dropdown-item.active {
  background: var(--gradient-primary);
  color: white;
}

.item-icon {
  font-size: 1rem;
}

/* Mobile Menu Button */
.mobile-menu-btn {
  display: none;
  flex-direction: column;
  gap: 5px;
  padding: 0.5rem;
  background: none;
  border: none;
  cursor: pointer;
}

.menu-line {
  width: 24px;
  height: 2px;
  background: var(--color-text);
  border-radius: var(--radius-full);
  transition: all var(--transition-base);
}

.menu-line.open:nth-child(1) {
  transform: rotate(45deg) translate(5px, 5px);
}

.menu-line.open:nth-child(2) {
  opacity: 0;
}

.menu-line.open:nth-child(3) {
  transform: rotate(-45deg) translate(5px, -5px);
}

/* Mobile Menu */
.mobile-menu {
  display: none;
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  padding: 1rem;
  border-bottom: 1px solid var(--color-border);
  transform: translateY(-100%);
  opacity: 0;
  visibility: hidden;
  transition: all var(--transition-base);
}

.mobile-menu.open {
  transform: translateY(0);
  opacity: 1;
  visibility: visible;
}

.mobile-nav-link {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  color: var(--color-text);
  text-decoration: none;
  font-weight: 500;
  border-radius: var(--radius-lg);
  transition: all var(--transition-base);
}

.mobile-nav-link:hover,
.mobile-nav-link.active {
  background: var(--gradient-primary);
  color: white;
}

.dark .mobile-menu {
  background: rgba(15, 23, 42, 0.95);
}

/* Mobile Controls */
.mobile-controls {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.mobile-control-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.mobile-control-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-soft);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.mobile-control-options {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.mobile-option-btn {
  padding: 0.5rem 0.875rem;
  background: var(--color-background-mute);
  border: 1px solid var(--color-border);
  border-radius: var(--radius);
  color: var(--color-text);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.mobile-option-btn:hover {
  border-color: var(--color-primary);
}

.mobile-option-btn.active {
  background: var(--gradient-primary);
  border-color: transparent;
  color: white;
}

/* Main Content */
.main-content {
  flex: 1;
  padding: 2rem;
}

.content-wrapper {
  max-width: 1400px;
  margin: 0 auto;
}

/* Page Transitions */
.page-enter-active,
.page-leave-active {
  transition: all var(--transition-slow);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

/* Footer */
.footer {
  padding: 1.5rem 2rem;
  border-top: 1px solid var(--color-border);
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(10px);
}

.dark .footer {
  background: rgba(15, 23, 42, 0.5);
}

.footer-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

.footer-text {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.footer-version {
  font-family: var(--font-mono);
  padding: 0.25rem 0.75rem;
  background: var(--color-background-mute);
  border-radius: var(--radius-full);
  font-size: 0.75rem;
}

/* Responsive */
@media (max-width: 1024px) {
  .nav-links {
    display: none;
  }

  .nav-controls {
    margin-left: auto;
  }

  .mobile-menu-btn {
    display: flex;
  }

  .mobile-menu {
    display: block;
  }
}

@media (max-width: 768px) {
  .main-content {
    padding: 1rem;
  }

  .footer-content {
    flex-direction: column;
    gap: 0.5rem;
    text-align: center;
  }

  .nav-container {
    padding: 0;
  }
}
</style>
