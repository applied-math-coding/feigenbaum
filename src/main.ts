import { createApp } from 'vue';
import './style.scss';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import 'primevue/resources/themes/saga-blue/theme.css';
import 'primevue/resources/primevue.min.css';
import 'primeicons/primeicons.css';
import 'primeflex/primeflex.css';
import 'mathjax/es5/tex-chtml.js';

createApp(App)
  .use(PrimeVue)
  .mount('#app');
