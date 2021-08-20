import { createApp } from 'vue';
import App from './App.vue';
import router from './config/router';
import store from './config/store';
import 'virtual:windi.css';
import 'virtual:windi-devtools';
import '~/assets/scss/fonts.scss';

const app = createApp(App);
app.use(router);
app.use(store);

app.mount('#app');
