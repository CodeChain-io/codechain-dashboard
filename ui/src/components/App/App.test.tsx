import * as React from "react";
import * as ReactDOM from "react-dom";
import { Provider } from "react-redux";
import { applyMiddleware, createStore } from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import thunkMiddleware from "redux-thunk";
import appReducer from "../../reducers";
import App from "./App";

it("renders without crashing", () => {
  const div = document.createElement("div");
  const composeEnhancers = composeWithDevTools({});
  const store = createStore(
    appReducer,
    composeEnhancers(applyMiddleware(thunkMiddleware))
  );
  ReactDOM.render(
    <Provider store={store as any}>
      <App />
    </Provider>,
    div
  );
  ReactDOM.unmountComponentAtNode(div);
});
