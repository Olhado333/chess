#![no_main]

use libfuzzer_sys::fuzz_target;
use chess::Board;

fuzz_target!(|data: &[u8]| {
    let fen = String::from_utf8_lossy(data);
    let _ = Board::from_fen(fen.to_string());
});