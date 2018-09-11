import * as React from "react";
import { withRouter } from "react-router-dom";
import "./Header.css";
import * as Logo from "./img/logo.png";

const getTitle = (pathName: string) => {
  switch (pathName) {
    case "/":
      return "CodeChain Dashboard";
    case "/nodelist":
      return "CodeChain Node List";
  }
  return "CodeChain";
};

export const Header = withRouter(props => {
  return (
    <div className="header d-flex align-items-center">
      <div className="logo-container text-center">
        <img className="animated fadeIn logo" src={Logo} />
      </div>
      <div className="title-container text-center">
        <h3 className="animated fadeIn mb-0">
          {getTitle(props.location.pathname)}
        </h3>
      </div>
      <div className="option-container" />
    </div>
  );
});
