///任意の式を表す
pub enum Expr {
    ConstantVal(ConstantVal),
    PlusOp(Box<PlusOp>),
}

impl Expr {
    ///式を評価する
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstantVal(e) => e.eval(),
            Expr::PlusOp(e) => e.eval()
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

///足し算を表す
pub struct PlusOp {
    //演算の左
    left_expr: Expr,
    //演算の右
    right_expr: Expr,
}

impl PlusOp {
    ///PlusOpを生成する
    pub fn new(left_expr: Expr, right_expr: Expr) -> PlusOp {
        PlusOp { left_expr, right_expr }
    }

    ///足し算を評価する
    pub fn eval(&self) -> i32 {
        self.left_expr.eval() + self.right_expr.eval()
    }
}

#[test]
fn plus_op_test() {
    //13+(5+1)の足し算式を生成
    let plus_op = PlusOp::new(
        Expr::ConstantVal(ConstantVal::new(13)),
        Expr::PlusOp(
            Box::new(
                PlusOp::new(
                    Expr::ConstantVal(ConstantVal::new(5)),
                    Expr::ConstantVal(ConstantVal::new(1)),
                )
            )
        ),
    );
    let expect = 13 + (5 + 1);
    assert_eq!(
        plus_op.eval(),
        expect
    );
}