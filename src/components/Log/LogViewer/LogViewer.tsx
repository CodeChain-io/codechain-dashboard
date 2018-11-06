import { faArrowDown } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import Table from "reactstrap/lib/Table";
import "./LogViewer.css";

export default class LogViewer extends React.Component<any, any> {
  public render() {
    return (
      <div className="log-viewer">
        <Table>
          <thead>
            <tr>
              <th>
                Date <FontAwesomeIcon icon={faArrowDown} />
              </th>
              <th>Node</th>
              <th>Status</th>
              <th>Type</th>
              <th>Body</th>
            </tr>
          </thead>
          <tbody>
            {_.map(_.range(10), item => (
              <tr key={item}>
                <td>2018-11-05 09:00</td>
                <td>Node {item}</td>
                <td>Error</td>
                <td>Type1</td>
                <td>Log Log Log Log Log Log Log Log Log Log</td>
              </tr>
            ))}
          </tbody>
        </Table>
      </div>
    );
  }
}
