pub mod calc;

fn main() {
    let a = calc::ast::ConstantVal::new(33);
    println!("Constant = {}", a.eval());
}