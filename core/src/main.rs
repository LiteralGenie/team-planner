mod console;
mod lib;

use console::log;
use splr::*;

fn main() {
    let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]];
    match Certificate::try_from(v) {
        Ok(Certificate::SAT(ans)) => log!("s SATISFIABLE: {:?}", ans),
        Ok(Certificate::UNSAT) => log!("s UNSATISFIABLE"),
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}
