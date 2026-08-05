use chess::{Board, ChessMove, MoveGen,Piece, Color, ALL_SQUARES, Square};





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


fn main(){

let mut board = Board::default();
let mut move_count = 0;


loop {


        let best = find_the_best_move_out_there_uhh_it_is_probably_not_the_best_but_shrug(&board,5);

        match best {
            None => {
                println!("game over after {} move",move_count);
                break;
            }
            Some(chosen) => {
                println!("Move {}: {}", move_count + 1, chosen);
                 board = board.make_move_new(chosen);
                move_count += 1;
            }
        }

                if move_count > 200 {
            println!("Stop after 200 moves");
            break;
        }
}

}