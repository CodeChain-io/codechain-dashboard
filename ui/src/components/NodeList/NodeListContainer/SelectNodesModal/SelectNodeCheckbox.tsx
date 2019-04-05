import * as React from "react";

interface Props {
  name: string;
  checked: boolean;
  onChange: (name: string) => void;
}

export default class SelectNodeCheckbox extends React.Component<Props> {
  public render() {
    const { name, checked } = this.props;
    return (
      <div>
        <label className="mr-1">{name}</label>
        <input
          type="checkbox"
          checked={checked}
          onChange={this.handleOnChange}
        />
      </div>
    );
  }

  private handleOnChange = () => {
    this.props.onChange(this.props.name);
  };
}
