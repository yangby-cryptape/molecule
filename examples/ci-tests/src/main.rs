use molecule::prelude::*;

use molecule_ci_tests::types;

fn main() {
    let data = types::AllInOne::default();
    println!("Data Length = {}\n", data.as_slice().len());
    println!("Data Debug:\n{:?}\n", data);
    println!("Data Display:\n{}\n", data);
}
