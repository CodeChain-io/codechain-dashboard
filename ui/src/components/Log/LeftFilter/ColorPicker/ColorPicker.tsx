import * as React from "react";
import { ColorState, GithubPicker } from "react-color";

import "./ColorPicker.css";

interface Props {
  onColorSelected: (hex: string) => void;
  className?: string;
  color: string;
}

interface State {
  isColorPickerOpen: boolean;
}

export default class ColorPicker extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      isColorPickerOpen: false
    };
  }
  public render() {
    const { isColorPickerOpen } = this.state;
    const { className, color } = this.props;
    return (
      <div className={`color-picker ${className}`}>
        <div
          className={`color-picker-button`}
          style={{ backgroundColor: color }}
          onClick={this.togglePicker}
        />

        {isColorPickerOpen && [
          <div className="block-picker-container" key="picker">
            <GithubPicker onChange={this.handleOnChangeColor} color={color} />
          </div>,
          <div key="cover" className="cover" onClick={this.handleClose} />
        ]}
      </div>
    );
  }
  public togglePicker = () => {
    this.setState({ isColorPickerOpen: !this.state.isColorPickerOpen });
  };
  public handleClose = () => {
    this.setState({ isColorPickerOpen: false });
  };
  public handleOnChangeColor = (color: ColorState) => {
    this.setState({
      isColorPickerOpen: false
    });
    this.props.onColorSelected(color.hex);
  };
}
