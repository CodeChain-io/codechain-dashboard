import * as React from "react";
import { connect, DispatchProp } from "react-redux";
import { Dispatch } from "redux";
import { Actions } from "../../actions";
import { RootState } from "../../reducers";
import { Apis } from "../../requests";
import { ChainNetworks } from "../../requests/types";
import { ConnectionGraphContainer } from "./ConnectGraphContainer/ConnectionGraphContainer";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
  getChainNetworks: () => void;
}
type Props = DispatchProp & OwnProps;
class Dashboard extends React.Component<Props> {
  public componentDidMount() {
    if (!this.props.chainNetworks) {
      this.props.getChainNetworks();
    }
  }
  public render() {
    const { chainNetworks } = this.props;
    if (!chainNetworks) {
      return <div>Loading...</div>;
    }
    return (
      <div className="dashboard vh-100">
        <ConnectionGraphContainer
          chainNetworks={chainNetworks}
          className="animated fadeIn"
        />
      </div>
    );
  }
}

const mapStateToProps = (state: RootState) => ({
  chainNetworks: state.chainNetworks
});
const mapDispatchToProps = (dispatch: Dispatch) => ({
  getChainNetworks: async () => {
    const chainNetworks = await Apis.getChainNetworks();
    dispatch(Actions.setChainNetworks(chainNetworks));
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Dashboard);
