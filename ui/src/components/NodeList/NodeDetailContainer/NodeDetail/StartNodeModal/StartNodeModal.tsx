import * as React from "react";
import * as Modal from "react-modal";
import { Form, Label } from "reactstrap";
import "./StartNodeModal.css";
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
  onStartNode: (env: string, args: string) => void;
  onAfterOpen: () => void;
  startOption?: {
    env: string;
    args: string;
  };
  isOpen: boolean;
}
interface State {
  env: string;
  args: string;
}

export default class StartNodeModal extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    const startOption = props.startOption;
    this.state = {
      env: startOption ? startOption.env : "",
      args: startOption ? startOption.args : ""
    };
  }
  public render() {
    const { isOpen, onAfterOpen, onClose, startOption } = this.props;
    const { env, args } = this.state;
    return (
      <div>
        <Modal
          isOpen={isOpen}
          onAfterOpen={onAfterOpen}
          onRequestClose={onClose}
          style={customStyles}
          contentLabel="Start node popup"
        >
          <Form className="start-node-modal-form animated fadeIn">
            <div className="form-group">
              <Label for="environment-variable-input">
                Environment variables
              </Label>
              <input
                type="text"
                className="form-control"
                id="environment-variable-input"
                aria-describedby="evnHelp"
                placeholder="Enter ENV"
                autoComplete="off"
                onChange={this.handleEnvChange}
                value={env}
              />
              <small id="evnHelp" className="form-text text-muted">
                {startOption ? startOption.env : ""}
              </small>
            </div>
            <div className="form-group">
              <Label for="argument-input">Arguments</Label>
              <input
                type="text"
                className="form-control"
                id="argument-input"
                aria-describedby="argHelp"
                placeholder="Enter Args"
                autoComplete="off"
                value={args}
                onChange={this.handleArgsChange}
              />
              <small id="argHelp" className="form-text text-muted">
                {startOption ? startOption.args : ""}
              </small>
            </div>
            <div className="d-flex justify-content-end">
              <button
                type="submit"
                onClick={this.onCloseClick}
                className="btn btn-secondary mr-3"
              >
                Cancel
              </button>
              <button
                type="submit"
                onClick={this.onSubmit}
                className="btn btn-primary"
              >
                Run
              </button>
            </div>
          </Form>
        </Modal>
      </div>
    );
  }

  private onCloseClick = (e: any) => {
    e.preventDefault();
    this.props.onClose();
  };

  private onSubmit = (e: any) => {
    e.preventDefault();
    const { args, env } = this.state;
    this.props.onStartNode(env, args);
  };

  private handleArgsChange = (event: any) => {
    this.setState({ args: event.target.value });
  };

  private handleEnvChange = (event: any) => {
    this.setState({ env: event.target.value });
  };
}
