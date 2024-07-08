///任意の式を表す
pub enum Expr {
    ConstantVal(ConstantVal),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    ///式を評価する
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstantVal(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval()
        }
    }
}

/// 定数を表す
pub struct ConstantVal(i32);

impl ConstantVal {
    ///ConstantValを生成する
    pub fn new(val: i32) -> ConstantVal {
        ConstantVal(val)
    }

    ///ConstantValの値を取得する
    pub fn eval(&self) -> i32 {
        self.0
    }
}

/// テストの実装
#[test]
fn Constant_val_test() {
    let expect = 55;
    let constant_val = ConstantVal::new(expect);
    assert_eq!(constant_val.eval(), expect);
}

///演算子種別
pub enum Opkind {
    Add,
    Sub,
    Mul,
    Div,
}

///二項円座因子を表す
pub struct BinaryOp {
    //適応する演算子種別
    op_kind: Opkind,
    //演算子の左にある式
    left_expr: Expr,
    //演算子の右にある式
    right_expr: Expr,
}

impl BinaryOp {
    ///BinaryOpを生成する
    pub fn new(left_expr: Expr, right_expr: Expr) -> BinaryOp {
        BinaryOp { left_expr, right_expr }
    }

    ///二項演算式を評価する
    pub fn eval(&self) -> i32 {
        let right = self.right_expr.eval();
        let left = self.left_expr.eval();
        mathch self.op_kind {
            Opkind::Add => left + right,
            Opkind::Sub => left - right,
            Opkind::Mul => left * right,
            Opkind::Div => left / right
        }
    }
}

#[test]
fn binary_op_test() {
    //13+(5+1)の式を生成
    let binary_op = BinaryOp::new(
        Opkind::Mul,
        Expr::ConstantVal(ConstantVal::new(13)),
        Expr::BinaryOp(
            Box::new(
                BinaryOp::new(
                    Expr::ConstantVal(ConstantVal::new(5)),
                    Expr::ConstantVal(ConstantVal::new(1)),
                )
            )
        ),
    );
    let expect = 13 + (5 + 1);
    assert_eq!(
        binary_op.eval(),
        expect
    );
}