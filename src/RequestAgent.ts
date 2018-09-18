import { Dispatch } from "redux";
const WebSocket = require("rpc-websockets").Client;

export default class RequestAgent {
  public static getInstance = () => {
    return RequestAgent.instance;
  };
  private static instance: RequestAgent = new RequestAgent();
  private ws: any;
  private dispatch: Dispatch;
  private serverHost = process.env.REACT_APP_SERVER_HOST
    ? process.env.REACT_APP_SERVER_HOST
    : "localhost:3012";
  private isConnected: boolean = false;
  constructor() {
    this.ws = new WebSocket(`ws://${this.serverHost}`);
    this.ws.on("open", () => {
      console.log("connected");
      this.isConnected = true;
      this.ws
        .subscribe(["dashboard_updated", "node_updated"])
        .catch((e: any) => {
          console.log(e);
        });

      this.ws.on("dashboard_updated", (e: any) => {
        console.log(e);
        this.dispatch({
          type: "DashboardUpdated"
        });
      });

      this.ws.on("node_updated", (e: any) => {
        console.log(e);
        this.dispatch({
          type: "NodeUpdated"
        });
      });
    });
    this.ws.on("error", (e: any) => {
      console.log("error", e);
    });
    this.ws.on("close", () => {
      console.log("closed");
    });
  }
  public setDispatch = (dispatch: Dispatch) => {
    this.dispatch = dispatch;
  };
  public call = async <T>(
    method: string,
    params: object | Array<object>
  ): Promise<T> => {
    await this.ensureConnection();
    return this.ws.call(method, params);
  };
  public close = () => {
    this.ws.close();
  };
  // Set timeout to 5 sec
  private ensureConnection() {
    let requestCount = 0;
    return new Promise((resolve, reject) => {
      (function waitForConnection() {
        if (RequestAgent.getInstance().isConnected) {
          return resolve();
        }
        if (requestCount < 100) {
          requestCount++;
          setTimeout(waitForConnection, 50);
        } else {
          return reject();
        }
      })();
    });
  }
}
