import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import './config/styles';
import router from './config/router';
import tooltip from './directives/tooltip';
import filters from './config/filters';
import money from './config/money';
import { loadConfigFile } from './config/config-file';



const app = createApp(App);
app.use(router);
app.use(createPinia());
app.use(tooltip);
app.use(filters);
app.use(money);

loadConfigFile()
  .then((isNewUser) => {
    if (isNewUser) {
      router.replace({
        name: 'home'
      })
    }
    app.mount('#app');
  });
