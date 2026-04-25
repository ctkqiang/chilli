import { createRouter, createWebHistory } from 'vue-router';
import DashboardView from '@/views/DashboardView.vue';
import ProcessesView from '@/views/ProcessesView.vue';
import SecurityView from '@/views/SecurityView.vue';
import DockerView from '@/views/DockerView.vue';
import IPAccessView from '@/views/IPAccessView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView
    },
    {
      path: '/processes',
      name: 'processes',
      component: ProcessesView
    },
    {
      path: '/security',
      name: 'security',
      component: SecurityView
    },
    {
      path: '/docker',
      name: 'docker',
      component: DockerView
    },
    {
      path: '/ip-access',
      name: 'ipAccess',
      component: IPAccessView
    }
  ]
});

export default router;
