import * as React from "react";
import * as Modal from "react-modal";
import "./UpgradeNodeModal.css";
const customStyles = {
  content: {
    top: "50%",
    left: "50%",
    right: "auto",
    bottom: "auto",
    marginRight: "-50%",
    transform: "translate(-50%, -50%)"
  }
};

interface Props {
  onClose: () => void;
  currentCommitHash: string;
  isOpen: boolean;
}

export default class UpgradeNodeModal extends React.Component<Props> {
  public constructor(props: Props) {
    super(props);
  }
  public render() {
    const { isOpen, onClose } = this.props;
    return (
      <div>
        <Modal
          isOpen={isOpen}
          onAfterOpen={this.onAfterOpen}
          onRequestClose={onClose}
          style={customStyles}
          contentLabel="Upgrade node popup"
        >
          <div className="upgrade-node-modal animated fadeIn">
            <div className="d-flex">
              <div className="branch-container">branch list</div>
              <div className="commit-container">Commit list</div>
            </div>
            <div className="d-flex justify-content-end">
              <button
                type="submit"
                onClick={this.onCloseClick}
                className="btn btn-secondary mt-3"
              >
                Cancel
              </button>
            </div>
          </div>
        </Modal>
      </div>
    );
  }

  private onCloseClick = (e: any) => {
    e.preventDefault();
    this.props.onClose();
  };

  private onAfterOpen = () => {
    console.log("Open");
  };
}
