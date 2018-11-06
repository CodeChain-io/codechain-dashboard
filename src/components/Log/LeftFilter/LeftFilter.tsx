import * as _ from "lodash";
import * as React from "react";
import { Label } from "reactstrap";
import "./LeftFilter.css";

export default class LeftFilter extends React.Component<any, any> {
  public render() {
    return (
      <div className="left-filter">
        <h5>Node filter</h5>
        <div>
          <ul className="list-unstyled">
            {_.map(_.range(3), item => (
              <li key={item}>
                <div className="form-check">
                  <input
                    type="checkbox"
                    className="form-check-input"
                    id="exampleCheck1"
                    checked={true}
                  />
                  <Label class="form-check-label" for="exampleCheck1">
                    Node {item}
                  </Label>
                </div>
              </li>
            ))}
          </ul>
          <hr />
        </div>
        <h5>Status filter</h5>
        <div>
          <ul className="list-unstyled">
            {_.map(["Error", "Warn", "Info"], item => (
              <li key={item}>
                <div className="form-check">
                  <input
                    type="checkbox"
                    className="form-check-input"
                    id="exampleCheck1"
                    checked={true}
                  />
                  <Label class="form-check-label" for="exampleCheck1">
                    {item}
                  </Label>
                </div>
              </li>
            ))}
          </ul>
          <hr />
        </div>
        <h5>Type filter</h5>
        <div>
          <ul className="list-unstyled">
            {_.map(["Type1", "Type2", "Type3"], item => (
              <li key={item}>
                <div className="form-check">
                  <input
                    type="checkbox"
                    className="form-check-input"
                    id="exampleCheck1"
                    checked={true}
                  />
                  <Label class="form-check-label" for="exampleCheck1">
                    {item}
                  </Label>
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    );
  }
}
