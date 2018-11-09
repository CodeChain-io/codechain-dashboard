import {
  faAngleDown,
  faAngleRight,
  faCopy
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import * as CopyToClipboard from "react-copy-to-clipboard";
import { connect } from "react-redux";
import { ReducerConfigure } from "../../../../reducers";
import { Log } from "../../../../requests/types";
import "./LogItem.css";

interface OwnProps {
  log: Log;
}

interface StateProps {
  nodeColors: {
    [nodeName: string]: string;
  };
}

interface State {
  isExpended: boolean;
  isCopied: boolean;
}

type Props = OwnProps & StateProps;

class LogItem extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      isExpended: false,
      isCopied: false
    };
  }
  public render() {
    const { log, nodeColors } = this.props;
    const { isExpended, isCopied } = this.state;

    const getItem = () => {
      return (
        <tr
          onClick={this.toggleExpend}
          className={"log-item animated fadeIn"}
          key="item-body"
          style={
            nodeColors[log.nodeName]
              ? {
                  backgroundColor: nodeColors[log.nodeName],
                  color: this.getColorByBackground(nodeColors[log.nodeName])
                }
              : {
                  backgroundColor: "#ffffff",
                  color: "#000000"
                }
          }
        >
          <td className="text-center">
            <span className="mr-3">
              {isExpended ? (
                <FontAwesomeIcon icon={faAngleDown} />
              ) : (
                <FontAwesomeIcon icon={faAngleRight} />
              )}
            </span>
            {moment(log.timestamp).format("YYYY-MM-DD HH:mm:ss")}
          </td>
          <td className="text-center">{log.nodeName}</td>
          <td className="text-center">{log.level}</td>
          <td className="text-center">{log.target}</td>
          <td className="message-col">{log.message}</td>
        </tr>
      );
    };
    const getExpendText = () => {
      return `${log.nodeName}, ${log.level}, ${log.target}, ${moment(
        log.timestamp
      ).format("YYYY-MM-DD HH:mm:ss")} content is\r\n\r\n${log.message}`;
    };
    if (isExpended) {
      return [
        getItem(),
        <tr key="item-expended" className="expended-item">
          <td colSpan={5}>
            <div className="expend-text">{getExpendText()}</div>
            <div className="mt-3">
              <CopyToClipboard
                text={getExpendText()}
                onCopy={this.handleOnCopy}
              >
                <button className="btn btn-secondary" type="button">
                  <FontAwesomeIcon icon={faCopy} className="mr-3" /> Copy to
                  clipboard
                </button>
              </CopyToClipboard>
              {isCopied && <span className="ml-3">Copied!</span>}
            </div>
          </td>
        </tr>
      ];
    } else {
      return getItem();
    }
  }
  private handleOnCopy = () => {
    this.setState({ isCopied: true });
  };
  private getColorByBackground = (hexBackground: string) => {
    const brightness = this.getBrightness(this.hexToRgb(hexBackground));
    return brightness > 0.5 ? "#000000" : "#ffffff";
  };
  private getBrightness = (rgb: number[]) => {
    const R = rgb[0] / 255;
    const G = rgb[1] / 255;
    const B = rgb[2] / 255;
    return Math.sqrt(
      0.299 * Math.pow(R, 2.2) +
        0.587 * Math.pow(G, 2.2) +
        0.114 * Math.pow(B, 2.2)
    );
  };
  private hexToRgb = (hex: string) => {
    return hex
      .replace(
        /^#?([a-f\d])([a-f\d])([a-f\d])$/i,
        (m, r, g, b) => "#" + r + r + g + g + b + b
      )
      .substring(1)
      .match(/.{2}/g)!
      .map(x => parseInt(x, 16));
  };

  private toggleExpend = () => {
    this.setState({ isExpended: !this.state.isExpended });
  };
}

const mapStateToProps = (state: ReducerConfigure) => ({
  nodeColors: state.logReducer.nodeColor
});
export default connect(mapStateToProps)(LogItem);
