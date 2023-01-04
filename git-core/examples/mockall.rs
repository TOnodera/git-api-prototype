use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

#[test]
fn mockall_test() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo().return_const(5 as u32);
    assert_eq!(5, mock.foo(0));
}

fn main() {}
