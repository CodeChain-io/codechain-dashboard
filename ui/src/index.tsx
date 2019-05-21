import "bootstrap/dist/css/bootstrap.min.css";
import * as React from "react";
import "react-confirm-alert/src/react-confirm-alert.css";
import * as ReactDOM from "react-dom";
import { Provider } from "react-redux";
import "react-widgets/dist/css/react-widgets.css";
import { applyMiddleware, createStore } from "redux";
import { composeWithDevTools } from "redux-devtools-extension/logOnlyInProduction";
import thunkMiddleware from "redux-thunk";
import App from "./components/App/App";
import appReducer from "./reducers";
import { unregister } from "./registerServiceWorker";
import "./styles/index.css";

const composeEnhancers = composeWithDevTools({});
const store = createStore(
  appReducer,
  composeEnhancers(applyMiddleware(thunkMiddleware))
);

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById("root") as HTMLElement
);
unregister();
