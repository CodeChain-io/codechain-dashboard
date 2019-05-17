import { toast } from "react-toastify";
import { updateChainNetworks } from "./actions/chainNetworks";
import { updateNodeInfo } from "./actions/nodeInfo";
import {
  ChainNetworksUpdate,
  CommonError,
  NodeUpdateInfo
} from "./requests/types";
const WebSocket = require("rpc-websockets").Client;

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
  private dispatch: any;
  private agentHubHost = process.env.REACT_APP_AGENT_HUB_HOST
    ? process.env.REACT_APP_AGENT_HUB_HOST
    : "ws://localhost:3012";
  private passphrase = process.env.REACT_APP_AGENT_HUB_PASSPHRASE
    ? process.env.REACT_APP_AGENT_HUB_PASSPHRASE
    : "passphrase";
  private isConnected: boolean = false;
  constructor() {
    console.log("Create websocket");
    this.ws = new WebSocket(this.agentHubHost + `/${this.passphrase}`);
    this.ws.on("open", () => {
      console.log("connected");
      this.isConnected = true;
      this.ws
        .subscribe(["dashboard_updated", "node_updated"])
        .catch((e: any) => {
          console.log(e);
        });

      this.ws.on("dashboard_updated", (e: ChainNetworksUpdate) => {
        this.dispatch(updateChainNetworks(e));
      });

      this.ws.on("node_updated", (e: NodeUpdateInfo) => {
        this.dispatch(updateNodeInfo(e.name, e));
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
  public setDispatch = (dispatch: any) => {
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
