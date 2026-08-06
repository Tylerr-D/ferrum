use chess::{Board, ChessMove, MoveGen,Piece, Color, ALL_SQUARES, Square};
use std::io;
use std::io::Write;
use std::str::FromStr;





fn evaluate(board: &Board) -> i32 {

    let mut score = 0;
    
    for square in ALL_SQUARES {

        if let Some(piece) = board.piece_on(square){

            let value = piece_value(piece);
            let color = board.color_on(square).unwrap();

            // sonion this is my code 

            if color == Color::White {
                score += value;
            }
            else {
                score -= value;
            }
            
        }
    }
    score
}


fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
    }
}


fn alphabeta(board:&Board, depth:u32, mut alpha: i32, mut beta: i32) -> i32 {

    if depth == 0 {
        return evaluate(board);
    }

    let legal_moves:Vec<ChessMove> = MoveGen::new_legal(board).collect();

    if legal_moves.is_empty() {
        if board.checkers().0 != 0 {
            return if board.side_to_move() == Color::White {
                -1_000_000
            } else {
                1_000_000
            };
        }
        else {
            return 0;
        }
    }

    if board.side_to_move() == Color::White {
        let mut best = i32::MIN;
        for m in legal_moves {
    
                let new_board = board.make_move_new(m);

                let score = alphabeta(&new_board, depth -1, alpha, beta);
                best = best.max(score);
                alpha = alpha.max(best);

                if beta <= alpha {
                    break;
                }
        }
        best
    }

    else {
                let mut best = i32::MAX;

                for m in legal_moves {

                let new_board = board.make_move_new(m);

                let score = alphabeta(&new_board, depth -1, alpha, beta);
                best = best.min(score);  
                beta = beta.min(best);

                if beta <= alpha {
    break;
}
    }
    best

    }
}

fn find_the_best_move_out_there_uhh_it_is_probably_not_the_best_but_shrug(board:&Board, depth:u32) -> Option<ChessMove>{

let legal_moves:Vec<ChessMove> = MoveGen::new_legal(board).collect();

if legal_moves.is_empty(){
    return None;
}

let mut best_move = legal_moves[0];


let mut best_score = if board.side_to_move() == Color::White {
        i32::MIN
    } 
    else {
        i32::MAX
    };

    let alpha = i32::MIN;
    let beta = i32::MAX;

    for m in legal_moves {
        let new_board = board.make_move_new(m);

        let score = alphabeta(&new_board, depth - 1, alpha, beta);
    
if board.side_to_move() == Color::White {
    if score > best_score {

    best_score = score;
    best_move = m;
    }
} else {

    if score < best_score {

        best_score = score;
        best_move = m;
    }
}

}

    Some(best_move)
}


fn print_board(board:&Board){
    // cleany clean woohoooo
    print!("\x1b[2J\x1b[3J\x1b[H");
io::stdout().flush().unwrap();

for rank in (0..8).rev(){
    print!(" {} ",rank +1);

    for file in 0..8 {

        let square = Square::make_square(
            chess::Rank::from_index(rank),
            chess::File::from_index(file),
        );

        let symbol = match board.piece_on(square){
            Some(piece) => {
                let c = board.color_on(square).unwrap();
                piece_char(piece,c)
            }

            None => '.'
        };

            print!(" {}  ", symbol);


    }

    println!();
    //clean stuff yk?
}

println!("    a   b   c   d   e   f   g   h");
//took time to figure out the spacing mb
println!();

}

fn piece_char(piece: Piece, color: Color) -> char {
 match (piece,color) {

    // took all these symbols from the net these help understanding whats going on the board got these 
    // from https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
 (Piece::King, Color::White) => '♔',
        (Piece::Queen, Color::White) => '♕',
        (Piece::Rook, Color::White) => '♖',
        (Piece::Bishop, Color::White) => '♗',
        (Piece::Knight, Color::White) => '♘',
        (Piece::Pawn, Color::White) => '♙',
        (Piece::King, Color::Black) => '♚',  
        (Piece::Queen, Color::Black) => '♛',
        (Piece::Rook, Color::Black) => '♜',
        (Piece::Bishop, Color::Black) => '♝',
        (Piece::Knight, Color::Black) => '♞',
        (Piece::Pawn, Color::Black) => '♟',
    }


}



fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
        input.trim().to_string()

}



fn main(){

    println!("welcome to ferrum! play as b(lack) or w(hite)");
    print!("> ");
    io::stdout().flush().unwrap();

    let choice = read_line().to_lowercase();

    let human_color = if choice.starts_with('b') {
                Color::Black
    }

    else {
                Color::White

    };

let search_depth: u32 = loop {

println!("1 - easy af (depth 2)");
println!("2 - easy (depth 3)");
println!("3 - a little less easy (depth 4)");
println!("4 - medium (depth 5)");
print!("> ");

io::stdout().flush().unwrap();

let difficulty = read_line();

    match difficulty.as_str() {


    "1" => break 2,
    "2" => break 3,
    "3" => break 4,
    "4" => break 5,
    _ => {
        println!("invalid choice\n");
    }
    }  
    };





let mut board = Board::default();

loop {

            print_board(&board);


                let legal_moves:Vec<ChessMove> = MoveGen::new_legal(&board).collect();

    if legal_moves.is_empty() {
        if board.checkers().0 != 0 {

            let winner = if board.side_to_move() == Color::White {
                "Black"
            }

            else {
                "White"
            };

                            println!("Checkmate! {} wins.", winner);

        }

                            else {
                                println!("stalemate - its a draw");
                            }
                            break;
        }




if board.side_to_move() == human_color {

    loop {
        print!("your move (e.g. e2e4): ");

        io::stdout().flush().unwrap();
         let input = read_line();

         match ChessMove::from_str(&input) {
            Ok(m) if legal_moves.contains(&m) =>  {
               board = board.make_move_new(m);
                break;
            }


            //rust ahh syntax
            _ => {
                println!("not a legal move");
            }
         }

    }
}

else {

    println!("engine is thinking");

    if let Some(m) = find_the_best_move_out_there_uhh_it_is_probably_not_the_best_but_shrug(&board, search_depth) {
        println!("engine plays {}",m);
        board = board.make_move_new(m);

    }
}
}
}