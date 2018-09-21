import * as _ from "lodash";
import * as React from "react";
import { ChainNetworks, NodeStatus } from "../../../requests/types";
const {
  InteractiveForceGraph,
  ForceGraphLink,
  ForceGraphNode
} = require("react-vis-force");

interface Props {
  className?: string;
  chainNetworks: ChainNetworks;
  onSelectNode: (node: { id: string; label: string }) => void;
  onDeselect: () => void;
}
interface States {
  width: number;
  height: number;
  drawNodeList: boolean;
  isUpdatingGraph: boolean;
}
export class ConnectionGraph extends React.Component<Props, States> {
  private containerRef: React.RefObject<HTMLDivElement>;
  constructor(props: Props) {
    super(props);
    this.state = {
      width: 0,
      height: 0,
      drawNodeList: false,
      isUpdatingGraph: false
    };
    this.containerRef = React.createRef();
  }

  public componentDidMount() {
    this.setWindowDimensions();
    window.addEventListener("resize", this.updateWindowDimensions);
  }

  public componentWillUnmount() {
    window.removeEventListener("resize", this.updateWindowDimensions);
  }

  public render() {
    const { className, chainNetworks, onSelectNode, onDeselect } = this.props;
    const { width, height, drawNodeList } = this.state;
    return (
      <div ref={this.containerRef} className={className}>
        {drawNodeList ? (
          <InteractiveForceGraph
            simulationOptions={{
              height,
              width,
              animate: true,
              radiusMargin: 30
            }}
            labelAttr="label"
            // tslint:disable-next-line:jsx-no-lambda
            onSelectNode={(event: any, node: any) => onSelectNode(node)}
            // tslint:disable-next-line:jsx-no-lambda
            onDeselectNode={(event: any, node: any) => onDeselect()}
            highlightDependencie={true}
          >
            {_.map(chainNetworks.nodes, node => (
              <ForceGraphNode
                key={`node-${node.address}`}
                node={{
                  id: node.address,
                  label: node.name ? node.name : node.address,
                  radius: 10
                }}
                showLabel={true}
                fill={this.getNodeColor(node.status)}
              />
            ))}
            {_.map(chainNetworks.connections, connection => (
              <ForceGraphLink
                link={{ source: connection.nodeA, target: connection.nodeB }}
              />
            ))}
          </InteractiveForceGraph>
        ) : null}
      </div>
    );
  }
  private getNodeColor = (nodeStatus: NodeStatus) => {
    switch (nodeStatus) {
      case "Run":
        return "#28a745";
      case "Error":
        return "#dc3545";
      case "Stop":
        return "#868e96";
      case "Starting":
        return "#ffc107";
      case "UFO":
        return "#17a2b8";
    }
    return "#868e96";
  };
  private setWindowDimensions = () => {
    this.setState({
      width: this.containerRef.current
        ? this.containerRef.current.offsetWidth
        : 500,
      height: this.containerRef.current
        ? this.containerRef.current.offsetHeight
        : 500,
      drawNodeList: true
    });
  };
  private updateWindowDimensions = () => {
    if (this.state.isUpdatingGraph) {
      return;
    }
    this.setState({
      drawNodeList: false,
      isUpdatingGraph: true
    });

    setTimeout(() => {
      this.setState({
        width: this.containerRef.current
          ? this.containerRef.current.offsetWidth
          : 500,
        height: this.containerRef.current
          ? this.containerRef.current.offsetHeight
          : 500,
        drawNodeList: true,
        isUpdatingGraph: false
      });
    }, 500);
  };
}
