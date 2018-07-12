import * as classNames from "classnames";
import * as React from "react";
import { SFC } from "react";
import "./board.css";
import { Backend } from "./contracts/backend";

export const Board: SFC<{ data: Backend }> = ({ data }) => (
    <div className="board-root">
        {data.board.map((rowData, row) => (
            <div key={row} className="board-row">
                {rowData.map((tileData, col) => (
                    <div
                        key={`${row},${col}`}
                        className={classNames("board-tile", {
                            alternate: !!((row ^ col) & 1)
                        })}
                    />
                ))}
            </div>
        ))}
    </div>
);

export default Board;
