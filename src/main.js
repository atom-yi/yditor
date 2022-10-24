import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import store from './store';
import { Button, Layout, List} from 'ant-design-vue';
import 'ant-design-vue/dist/antd.css';

const app = createApp(App);
app.mount('#app');
app.use(store);
app.use(Button);
app.use(Layout);
app.use(List);
