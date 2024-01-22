pub mod util;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, time};
use util::board::Board;

const N: usize = 13;

fn check_occupation(board: &Board, column: usize, y: usize) -> bool {
    for x in 0..column.saturating_sub(1) {
        if board.get(x, y).is_occupied()
            || (y + x >= column && board.get(x, y + x - column).is_occupied())
            || (y + column < N + x && board.get(x, y + column - x).is_occupied())
        {
            return false;
        }
    }
    true
}

fn solve_parallel(board: &mut Board, column: usize, count: &Arc<Mutex<i64>>, y: usize) {
    if check_occupation(board, column, y) {
        board.set(column, y, true);
        if column == N - 1 {
            *count.lock().unwrap() += 1;
            return;
        }

        for z in (0..N).filter(|&z| z != y && z != y.saturating_sub(1) && z != (y + 1).min(N - 1)) {
            solve_parallel(board, column + 1, count, z);
        }

        board.set(column, y, false);
    }
}

fn solve(board: Board, column: usize, count: &mut i64) {
    let count_mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for y in 0..N {
        let count_clone = Arc::clone(&count_mutex);
        let handle = thread::spawn(move || {
            let mut board_clone = board;
            solve_parallel(&mut board_clone, column, &count_clone, y);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *count = count_mutex.lock().unwrap().clone();
}

fn main() {
    let board = Board::new();
    let now = time::Instant::now();
    let mut count: i64 = 0;
    solve(board, 0, &mut count);
    println!("{}\n{}", now.elapsed().as_nanos(), count);
}
