import { Component } from "react";
import * as React from "react";
import "./GlobalNavigationBar.css";

export default class GlobalNavigationBar extends Component {
  constructor(props: {}) {
    super(props);
  }

  public render() {
    return <div className="global-navigation-bar">Navi</div>;
  }
}
