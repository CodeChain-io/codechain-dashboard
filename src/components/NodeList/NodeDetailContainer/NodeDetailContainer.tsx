import * as React from "react";
import { connect } from "react-redux";

class NodeDetailContainer extends React.Component {
  public render() {
    return (
      <div>
        <div>
          <div>Status : Run</div>
          <div>
            <button type="button" className="btn btn-danger">
              Stop
            </button>
          </div>
        </div>
      </div>
    );
  }
}

export default connect()(NodeDetailContainer);
