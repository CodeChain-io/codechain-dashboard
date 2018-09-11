import "bootstrap/dist/css/bootstrap.min.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import App from "./App";
import registerServiceWorker from "./registerServiceWorker";
import "./styles/index.css";

ReactDOM.render(<App />, document.getElementById("root") as HTMLElement);
registerServiceWorker();
