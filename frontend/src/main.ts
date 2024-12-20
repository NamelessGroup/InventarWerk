import './style.css'

import { createApp } from 'vue'
import App from './App.vue'
import { createPinia } from 'pinia'
import { ErrorHandler } from './errorHandling/ErrorHandler'

ErrorHandler.getInstance().addListener((e) => console.error(e))

const app = createApp(App)
app.use(createPinia())
app.config.errorHandler = (err, _, info) => {
  console.error(err)
  console.info(info)
  ErrorHandler.getInstance().registerError(err as Error)
}
app.mount('#app')
