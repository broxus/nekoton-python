import './main.scss';
import 'vue-toastification/dist/index.css';
// Theme components
import DefaultTheme from 'vitepress/theme';

import Toast from 'vue-toastification';
import BDKSimpleToast from './components/BDKSimpleToast.vue';

import BDKLayout from './components/BDKLayout.vue';
import BDKOutlineComponent from './components/shared/outline/BDKOutline.vue';
import BDKOutlineItem from './components/shared/outline/BDKOutlineItem.vue';
import BDKAccordionComponent from './components/shared/BDKAccordion.vue';
import BDKDisconnectIcon from './components/shared/BDKDisconnectIcon.vue';
import BDKImgContainer from './components/shared/BDKImgContainer.vue';

import { toast } from '../../src/helpers';

export default {
  ...DefaultTheme,
  Layout: BDKLayout,
  enhanceApp({ app }) {
    DefaultTheme.enhanceApp({ app });
    app.use(Toast, {
      position: 'top-right',
      timeout: 5000,
      closeOnClick: false,
      pauseOnFocusLoss: true,
      pauseOnHover: true,
      draggable: true,
      draggablePercent: 0.7,

      showCloseButtonOnHover: false,
      hideProgressBar: false,
      closeButton: 'button',
      icon: true,
      rtl: false,
    });
    app.config.errorHandler = function (err, vm, info) {
      toast(err.message, 0);
    };

    app.component('BDKSimpleToast', BDKSimpleToast);
    app.component('BDKOutline', BDKOutlineComponent);
    app.component('BDKOutlineItem', BDKOutlineItem);
    app.component('BDKDisconnectIcon', BDKDisconnectIcon);
    app.component('BDKAccordion', BDKAccordionComponent);
    app.component('BDKImgContainer', BDKImgContainer);
  },
};
