use splr::*;
mod console;

// fn main() {
//     let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]];
//     match Certificate::try_from(v) {
//         Ok(Certificate::SAT(ans)) => println!("s SATISFIABLE: {:?}", ans),
//         Ok(Certificate::UNSAT) => println!("s UNSATISFIABLE"),
//         Err(e) => panic!("s UNKNOWN; {}", e),
//     }
// }

fn main() {
    console::log!("Hello world!")
}