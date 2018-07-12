import * as React from "react";
import { SFC } from "react";
import black_pawn from "./assets/tile000.png";
import black_knight from "./assets/tile001.png";
import black_bishop from "./assets/tile002.png";
import black_rook from "./assets/tile003.png";
import black_queen from "./assets/tile004.png";
import black_king from "./assets/tile005.png";
import white_pawn from "./assets/tile006.png";
import white_knight from "./assets/tile007.png";
import white_bishop from "./assets/tile008.png";
import white_rook from "./assets/tile009.png";
import white_queen from "./assets/tile010.png";
import white_king from "./assets/tile011.png";
import { PlayerPiece } from "./contracts/backend";

const imageMap = {
    black_bishop,
    black_king,
    black_knight,
    black_pawn,
    black_queen,
    black_rook,
    white_bishop,
    white_king,
    white_knight,
    white_pawn,
    white_queen,
    white_rook
};

const Tile: SFC<PlayerPiece> = props => (
    <img src={imageMap[`${props.player}_${props.piece}`]} />
);
export default Tile;
