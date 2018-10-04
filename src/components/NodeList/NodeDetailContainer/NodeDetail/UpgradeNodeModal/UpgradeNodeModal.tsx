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
  onUpdateNode: (hash: string) => void;
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
  inputCommit: string;
  selectedBranchName?: string;
  selectedTagName?: string;
}

export default class UpgradeNodeModal extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      selectedType: "upgrade-by-branch",
      branchList: [],
      tagList: [],
      isTagEmpty: false,
      inputCommit: "",
      selectedBranchName: undefined,
      selectedTagName: undefined
    };
  }
  public componentDidMount() {
    axios
      .get("https://api.github.com/repos/CodeChain-io/CodeChain/branches")
      .then((response: any) => {
        let selectedBranchName = "";
        if (
          _.find(response.data, (data: any) => data.name === "master") !==
          undefined
        ) {
          selectedBranchName = "master";
        } else {
          selectedBranchName = response.data[0].name;
        }
        this.setState({
          branchList: response.data,
          selectedBranchName
        });
      })
      .catch(err => console.log(err));
    axios
      .get("https://api.github.com/repos/CodeChain-io/CodeChain/tags")
      .then((response: any) => {
        if (response.data.length === 0) {
          this.setState({ isTagEmpty: true });
        }
        this.setState({
          tagList: response.data,
          selectedTagName: response.data[0].name
        });
      })
      .catch(err => console.log(err));
  }
  public render() {
    const { isOpen, onClose } = this.props;
    const {
      selectedType,
      branchList,
      tagList,
      isTagEmpty,
      inputCommit,
      selectedBranchName,
      selectedTagName
    } = this.state;
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
                    {branchList.length === 0 ? (
                      <span>Loading ... </span>
                    ) : (
                      <select
                        id="inputState"
                        className="form-control"
                        onChange={this.handleSelectingBranch}
                        value={selectedBranchName}
                      >
                        {_.map(branchList, branch => {
                          return (
                            <option key={branch.commit.sha}>
                              {branch.name}
                            </option>
                          );
                        })}
                      </select>
                    )}
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
                    {!isTagEmpty && tagList.length === 0 ? (
                      <span>Loading...</span>
                    ) : isTagEmpty ? (
                      <span>The tag does not exist.</span>
                    ) : (
                      <select
                        id="inputState"
                        className="form-control"
                        value={selectedTagName}
                        onChange={this.handleSelectingTag}
                      >
                        {_.map(tagList, tag => {
                          return (
                            <option key={tag.commit.sha}>{tag.name}</option>
                          );
                        })}
                      </select>
                    )}
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
                    value={inputCommit}
                    onChange={this.handleCommitInput}
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
                onClick={this.onUpgradeClick}
                className="btn btn-primary mt-3"
                disabled={this.getSelectedCommitHash() === ""}
              >
                Upgrade
              </button>
            </div>
          </div>
        </Modal>
      </div>
    );
  }

  private onUpgradeClick = (e: any) => {
    e.preventDefault();
    this.props.onUpdateNode(this.getSelectedCommitHash());
  };

  private getSelectedCommitHash = () => {
    const selectedType = this.state.selectedType;
    let commitHash = "";
    switch (selectedType) {
      case "upgrade-by-commit":
        commitHash = this.state.inputCommit;
        break;
      case "upgrade-by-branch":
        {
          const selectedBranchName = this.state.selectedBranchName;
          const selectedBranch = _.find(
            this.state.branchList,
            branch => branch.name === selectedBranchName
          );
          if (selectedBranch) {
            commitHash = selectedBranch.commit.sha;
          }
        }
        break;
      case "upgrade-by-tag":
        {
          const selectedTagName = this.state.selectedTagName;
          const selectedTag = _.find(
            this.state.tagList,
            tag => tag.name === selectedTagName
          );
          if (selectedTag) {
            commitHash = selectedTag.commit.sha;
          }
        }
        break;
    }
    return commitHash;
  };
  private handleCommitInput = (e: any) => {
    this.setState({ inputCommit: e.target.value });
  };
  private handleSelectingBranch = (e: any) => {
    this.setState({ selectedBranchName: e.target.value });
  };
  private handleSelectingTag = (e: any) => {
    this.setState({ selectedTagName: e.target.value });
  };
  private onCloseClick = (e: any) => {
    e.preventDefault();
    this.props.onClose();
  };

  private onAfterOpen = () => {
    console.log("Open");
  };
}
