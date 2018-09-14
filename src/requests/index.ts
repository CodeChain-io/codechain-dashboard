import Sockette from "sockette";

export default class RequestAgent {
  private ws: Sockette;
  constructor(serverHost: string = "localhost:3012") {
    this.ws = new Sockette(`ws://${serverHost}`, {
      timeout: 5e3,
      maxAttempts: 10,
      onopen: this.onOpen,
      onmessage: this.onMessage,
      onreconnect: this.onReconnect,
      onmaximum: this.onMaximum,
      onclose: this.onClose,
      onerror: this.onError
    });
  }
  public sendMessage = (message: string) => {
    this.ws.send(message);
  };
  public close = () => {
    this.ws.close();
  };
  private onOpen = (e: Event) => {
    console.log("Connected!", e);
  };
  private onMessage = (e: MessageEvent) => {
    console.log("Received:", e);
  };
  private onReconnect = (e: Event | CloseEvent) => {
    console.log("Reconnecting...", e);
  };
  private onMaximum = (e: CloseEvent) => {
    console.log("Stop Attempting!", e);
  };
  private onClose = (e: CloseEvent) => {
    console.log("Closed!", e);
  };
  private onError = (e: Event) => {
    console.log("Error:", e);
  };
}
