import { Money3Component } from 'v-money3';
import { App } from 'vue';

export const vMoneyConfig = {
  decimal: ',',
  thousands: '.',
  prefix: 'R$ ',
  suffix: '',
  precision: 2,
  masked: false,
  disableNegative: false,
  disabled: false,
  min: Number.MIN_SAFE_INTEGER,
  max: Number.MAX_SAFE_INTEGER,
  allowBlank: false,
  minimumNumberOfCharacters: 0
};

export default {
  install
}

function install(app: App) {
  app.component('money', Money3Component);

  if (app.config.globalProperties.$configs == null) {
    app.config.globalProperties.$configs = {};
  }

  app.config.globalProperties.$configs.vMoney = vMoneyConfig;
}
