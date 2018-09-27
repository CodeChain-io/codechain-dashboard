import { NodeStatus } from "../requests/types";

export const getStatusClass = (status: NodeStatus) => {
  switch (status) {
    case "Run":
      return "text-success";
    case "Stop":
      return "text-secondary";
    case "Error":
      return "text-danger";
    case "Starting":
      return "text-warning";
    case "UFO":
      return "text-info";
  }
  throw new Error("Invalid status");
};
