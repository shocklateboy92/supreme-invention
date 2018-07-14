#[macro_use] extern crate nickel;
#[macro_use] extern crate serde_derive;

extern crate nickel_cors;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate chrono;

use nickel::{Nickel/*, StaticFilesHandler*/};
use hyper::header::AccessControlAllowOrigin;

#[derive(Serialize,Debug,Clone,Copy)]
#[serde(rename_all="camelCase")]
#[allow(dead_code)]
enum Player {
    Black,
    White
}

impl Default for Player {
    fn default() -> Self {
        Player::Black
    }
}

#[derive(Serialize,Debug,Clone,Copy)]
#[serde(rename_all="camelCase")]
#[allow(dead_code)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Pawn
    }
}

#[derive(Serialize,Default,Debug,Copy,Clone)]
struct PlayerPiece {
    player: Player,
    piece: Piece,
}

#[derive(Serialize,Default,Debug)]
struct StateRepr {
    board: [[Option<PlayerPiece>;8]; 8]
}

// Hack to enable CORS, use on all requests
fn cors_hack(res: &mut nickel::Response) {
    res.headers_mut().set(AccessControlAllowOrigin::Any);
}

fn main() {
    let mut srv = Nickel::new();
    let mut state: StateRepr = Default::default();
    state.board[3][4] = Some(Default::default());
    state.board[7][7] = Some(PlayerPiece {
        player: Player::White,
        piece: Piece::Queen,
    });

    //srv.utilize(nickel_cors::enable_cors);
    srv.utilize(router! {
        get "/" => |req, mut res| {
            let now = chrono::prelude::Local::now();
            println!(
                "Request from {} at {}", 
                req.origin.remote_addr,
                now
            );
            cors_hack(&mut res);
            serde_json::to_string(&state).unwrap()
        }
    });
    //srv.utilize(StaticFilesHandler::new("assets/"));

    let addr = "0.0.0.0";
    let port = "8080";
    let srv_addr = format!("{}:{}", addr, port);

    srv.listen(srv_addr).unwrap();
}
