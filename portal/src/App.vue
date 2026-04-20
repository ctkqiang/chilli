<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import { ref, onMounted, onUnmounted } from 'vue';

const route = useRoute();
const scrolled = ref(false);
const mobileMenuOpen = ref(false);

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
  { path: '/', name: '仪表盘', icon: '📊' },
  { path: '/processes', name: '进程', icon: '⚡' },
  { path: '/security', name: '安全', icon: '🛡️' },
  { path: '/docker', name: 'Docker', icon: '🐳' },
];

const isActive = (path: string) => route.path === path;
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
            <span class="brand-name">Chilli</span>
            <span class="brand-tagline">System Monitor</span>
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
            <span class="nav-text">{{ link.name }}</span>
            <div class="nav-indicator"></div>
          </router-link>
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
          <span class="nav-text">{{ link.name }}</span>
        </router-link>
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
        <span class="footer-text">Made with 💜 by Chilli Team</span>
        <span class="footer-version">v1.0.0</span>
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

@media (prefers-color-scheme: dark) {
  .navbar.scrolled {
    background: rgba(15, 23, 42, 0.8);
    border-bottom-color: rgba(51, 65, 85, 0.6);
  }
}

.nav-container {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
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

@media (prefers-color-scheme: dark) {
  .mobile-menu {
    background: rgba(15, 23, 42, 0.95);
  }
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

@media (prefers-color-scheme: dark) {
  .footer {
    background: rgba(15, 23, 42, 0.5);
  }
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
@media (max-width: 768px) {
  .nav-links {
    display: none;
  }

  .mobile-menu-btn {
    display: flex;
  }

  .mobile-menu {
    display: block;
  }

  .main-content {
    padding: 1rem;
  }

  .footer-content {
    flex-direction: column;
    gap: 0.5rem;
    text-align: center;
  }
}
</style>