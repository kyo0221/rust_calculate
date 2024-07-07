/// 定数を表す
pub struct ConstantVal(i32);

impl ConstantVal {
    pub fn new(val: i32) -> ConstantVal {
        ConstantVal(val)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

/// テストの実装
#[test]
fn Constant_val_test() {
    let expect = 55;
    let constant_val = ConstantVal::new(expect);
    assert_eq!(constant_val.get(), expect);
}