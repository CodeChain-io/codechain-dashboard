import axios from "axios";
import * as _ from "lodash";
import * as React from "react";
import * as Modal from "react-modal";
import { Label } from "reactstrap";
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

interface State {
  selectedType: string;
  branchList: {
    name: string;
    commit: {
      sha: string;
      url: string;
    };
  }[];
  tagList: {
    name: string;
    commit: {
      sha: string;
      url: string;
    };
  }[];
  isTagEmpty: boolean;
}

export default class UpgradeNodeModal extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      selectedType: "upgrade-by-branch",
      branchList: [],
      tagList: [],
      isTagEmpty: false
    };
  }
  public componentDidMount() {
    axios
      .get("https://api.github.com/repos/CodeChain-io/CodeChain/branches")
      .then((response: any) => {
        this.setState({ branchList: response.data });
      })
      .catch(err => console.log(err));
    axios
      .get("https://api.github.com/repos/CodeChain-io/CodeChain/tags")
      .then((response: any) => {
        if (response.data.length === 0) {
          this.setState({ isTagEmpty: true });
        }
        this.setState({ tagList: response.data });
      })
      .catch(err => console.log(err));
  }
  public render() {
    const { isOpen, onClose } = this.props;
    const { selectedType, branchList, tagList, isTagEmpty } = this.state;
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
            <div>
              <div>
                <div className="form-check">
                  <input
                    type="radio"
                    className="form-check-input"
                    id="upgrade-by-branch"
                    name="upgrade-type"
                    value="upgrade-by-branch"
                    checked={selectedType === "upgrade-by-branch"}
                    // tslint:disable-next-line:jsx-no-lambda
                    onChange={e =>
                      this.setState({ selectedType: e.target.value })
                    }
                  />
                  <Label className="form-check-label" for="upgrade-by-branch">
                    Upgrade by branch
                  </Label>
                </div>
                {selectedType === "upgrade-by-branch" && (
                  <div className="mt-3 mb-3">
                    <select id="inputState" className="form-control">
                      {branchList.length === 0 ? (
                        <option selected={true}>Loading...</option>
                      ) : (
                        _.map(branchList, branch => {
                          return (
                            <option key={branch.commit.sha}>
                              {branch.name}
                            </option>
                          );
                        })
                      )}
                    </select>
                  </div>
                )}
              </div>
              <div>
                <div className="form-check">
                  <input
                    type="radio"
                    className="form-check-input"
                    id="upgrade-by-tag"
                    name="upgrade-type"
                    value="upgrade-by-tag"
                    checked={selectedType === "upgrade-by-tag"}
                    // tslint:disable-next-line:jsx-no-lambda
                    onChange={e =>
                      this.setState({ selectedType: e.target.value })
                    }
                  />
                  <Label className="form-check-label" for="upgrade-by-tag">
                    Upgrade by tag
                  </Label>
                </div>
                {selectedType === "upgrade-by-tag" && (
                  <div className="mt-3 mb-3">
                    <select id="inputState" className="form-control">
                      {!isTagEmpty && tagList.length === 0 ? (
                        <option selected={true}>Loading...</option>
                      ) : isTagEmpty ? (
                        <option selected={true}>Empty</option>
                      ) : (
                        _.map(tagList, tag => {
                          return (
                            <option key={tag.commit.sha}>{tag.name}</option>
                          );
                        })
                      )}
                    </select>
                  </div>
                )}
              </div>
              <div>
                <div className="form-check">
                  <input
                    type="radio"
                    className="form-check-input"
                    id="upgrade-by-commit"
                    name="upgrade-type"
                    value="upgrade-by-commit"
                    checked={selectedType === "upgrade-by-commit"}
                    // tslint:disable-next-line:jsx-no-lambda
                    onChange={e =>
                      this.setState({ selectedType: e.target.value })
                    }
                  />
                  <Label className="form-check-label" for="upgrade-by-commit">
                    Upgrade by commit hash
                  </Label>
                </div>
              </div>
              {selectedType === "upgrade-by-commit" && (
                <div className="form-group mt-3 mb-3">
                  <input
                    type="text"
                    className="form-control"
                    placeholder="Commit hash"
                  />
                </div>
              )}
            </div>
            <div className="d-flex justify-content-end">
              <button
                type="submit"
                onClick={this.onCloseClick}
                className="btn btn-secondary mt-3 mr-3"
              >
                Cancel
              </button>
              <button
                type="submit"
                onClick={this.onCloseClick}
                className="btn btn-primary mt-3"
              >
                Upgrade
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
