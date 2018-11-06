import * as React from "react";
import { ColorResult, GithubPicker } from "react-color";

import "./ColorPicker.css";

interface Props {
  onColorSelected: (hex: string) => void;
  className?: string;
}

interface State {
  isColorPickerOpen: boolean;
  currentSelectedColor: string;
}

export default class ColorPicker extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      isColorPickerOpen: false,
      currentSelectedColor: "#ffffff"
    };
  }
  public render() {
    const { isColorPickerOpen, currentSelectedColor } = this.state;
    const { className } = this.props;
    return (
      <div className={`color-picker ${className}`}>
        <div
          className={`color-picker-button`}
          style={{ backgroundColor: currentSelectedColor }}
          onClick={this.togglePicker}
        />

        {isColorPickerOpen && [
          <div className="block-picker-container" key="picker">
            <GithubPicker
              onChange={this.handleOnChangeColor}
              color={currentSelectedColor}
            />
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
  public handleOnChangeColor = (color: ColorResult) => {
    this.setState({
      currentSelectedColor: color.hex,
      isColorPickerOpen: false
    });
    this.props.onColorSelected(color.hex);
  };
}
