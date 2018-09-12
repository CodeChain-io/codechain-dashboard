import * as React from "react";
const {
  InteractiveForceGraph,
  ForceGraphLink,
  ForceGraphNode
} = require("react-vis-force");

interface Props {
  className?: string;
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
    const { className } = this.props;
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
            onSelectNode={(node: any) => console.log(node)}
            highlightDependencie={true}
          >
            <ForceGraphNode
              node={{ id: "node1", label: "Node-1", radius: 10 }}
              showLabel={true}
              fill="red"
            />
            <ForceGraphNode
              node={{ id: "node2", label: "Node-2", radius: 10 }}
              showLabel={true}
              fill="blue"
            />
            <ForceGraphNode
              node={{ id: "node3", label: "Node-3", radius: 10 }}
              showLabel={true}
              fill="red"
            />
            <ForceGraphNode
              node={{ id: "node4", label: "Node-4", radius: 10 }}
              showLabel={true}
              fill="red"
            />
            <ForceGraphLink link={{ source: "node1", target: "node2" }} />
            <ForceGraphLink link={{ source: "node1", target: "node3" }} />
            <ForceGraphLink link={{ source: "node1", target: "node4" }} />
            <ForceGraphLink link={{ source: "node3", target: "node4" }} />
            <ForceGraphLink link={{ source: "node3", target: "node2" }} />
          </InteractiveForceGraph>
        ) : null}
      </div>
    );
  }

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
