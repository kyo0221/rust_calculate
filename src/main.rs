pub mod calc;

fn main() {
    let a=calc::ast::ConstantVal(33);
    println!("ConstantVal={}",a.0);
}
