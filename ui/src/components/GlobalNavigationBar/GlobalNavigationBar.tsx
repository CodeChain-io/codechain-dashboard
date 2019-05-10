import {
  faChartLine,
  faCoins,
  faHistory,
  faRetweet,
  faTachometerAlt,
  IconDefinition
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as React from "react";
import { Link, withRouter } from "react-router-dom";
import "./GlobalNavigationBar.css";
import * as arrowImg from "./img/arrow.svg";
const getGnbMenu = (
  url: string,
  title: string,
  icon: IconDefinition,
  isSelected: boolean
) => {
  return (
    <li className="animated fadeIn">
      <Link to={`/${url}`}>
        <div className={`gnb-list-item ${isSelected ? "active" : null}`}>
          {isSelected ? (
            <img className="gnb-list-item-selected-arrow" src={arrowImg} />
          ) : null}
          <div className="gnb-list-item-icon text-center">
            <FontAwesomeIcon icon={icon} />
          </div>
          <div className="gnb-list-item-title text-center">
            <span>{title}</span>
          </div>
        </div>
      </Link>
    </li>
  );
};
export const GlobalNavigationBar = withRouter(props => {
  const pathname = props.location.pathname;
  return (
    <div className="global-navigation-bar">
      <ul className="gnb-list list-unstyled">
        {getGnbMenu("", "Dashboard", faTachometerAlt, pathname === "/")}
        {getGnbMenu(
          "nodelist",
          "Node List",
          faCoins,
          /^\/nodelist/.test(pathname)
        )}
        {getGnbMenu("rpc", "RPC", faRetweet, pathname === "/rpc")}
        {getGnbMenu("log", "Log", faHistory, pathname === "/log")}
        {getGnbMenu("graph", "Graph", faChartLine, pathname === "/graph")}
      </ul>
    </div>
  );
});
