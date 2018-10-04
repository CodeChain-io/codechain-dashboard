import { Dispatch } from "redux";
import { Actions } from "./actions";
import {
  ChainNetworksUpdate,
  CommonError,
  NodeUpdateInfo
} from "./requests/types";
const WebSocket = require("rpc-websockets").Client;
import { toast } from "react-toastify";

export interface JsonRPCError {
  code: number;
  message: string;
}

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

      this.ws.on("dashboard_updated", (e: ChainNetworksUpdate) => {
        this.dispatch(Actions.updateChainNetworks(e));
      });

      this.ws.on("node_updated", (e: NodeUpdateInfo) => {
        this.dispatch(Actions.updateNodeInfo(e.name, e));
      });
    });
    this.ws.on("error", (e: any) => {
      console.log("error", e);
    });
    this.ws.on("close", () => {
      toast.error("Agent hub is closed.");
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
    try {
      await this.ensureConnection();
    } catch (e) {
      toast.error("Agent hub is not responding.");
      throw e;
    }
    let response;
    try {
      response = await this.ws.call(method, params);
    } catch (e) {
      if (!this.handleCommonError(e)) {
        throw e;
      }
    }
    return response;
  };
  public close = () => {
    this.ws.close();
  };
  private handleCommonError = (e: JsonRPCError) => {
    switch (e.code) {
      case CommonError.AgentNotFound:
        toast.error("Agent not found");
        return true;
      case CommonError.CodeChainIsNotRunning:
        toast.error("CodeChain is not running.");
        return true;
      case CommonError.InternalError:
        toast.error("Internal error");
        return true;
    }
    return false;
  };
  // Set timeout to 5 sec
  private ensureConnection = () => {
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
  };
}
