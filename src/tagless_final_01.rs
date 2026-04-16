// In our intial encoding example, we have to modify our AST if we want to extend it,
// so it is not as "open" as it could be. For example, if we want to add a new variant "Multiply"
// to our AST, then every function "eval", "pretty-print", etc will break.
// This is known as the "expression problem".
//
// In contrast to this a "final-encoding"/"tagless-final" encoding
// represents the data as functions (or traits in Rust)
// rather than data-types/ASTs. This solves the expression problem
// because it means new "variants" such as "Multiply" simply
// create new traits and don't modify existing code. "Extend, don't modify".
//
// So instead of writing functions for an AST, we create "interpreters" of a trait
//
// It's called "tagless" because there are no AST "tags" like `Add` or `Val`
// It's called "final" because we define the eDSL by its semantics (i.e. by its interpretation)
//
// Overall, initial encoding makes it easy to add new interpreters,
// but expensive to add new language variants; whereas, final/tagless inverts that.

pub trait Calculator {
  type Repr;

  fn val(v: i32) -> Self::Repr;
  fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr;
  fn sub(a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

pub struct Eval;

pub struct PrettyPrint;

impl Calculator for Eval {
  type Repr = i32;

  fn val(v: i32) -> Self::Repr {
    todo!()
  }

  fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }

  fn sub(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }
}

impl Calculator for PrettyPrint {
  type Repr = String;

  fn val(v: i32) -> Self::Repr {
    todo!()
  }

  fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }

  fn sub(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }
}

fn add_10_and_10<C: Calculator>() -> C::Repr {
  todo!()
}

#[cfg(test)]
mod test {
  use crate::tagless_final_01::{Calculator, Eval, PrettyPrint, add_10_and_10};

  #[test]
  fn test_can_add() {
    assert_eq!(
      Eval::val(6),
      Eval::add(Eval::val(1), Eval::add(Eval::val(2), Eval::val(3)))
    )
  }

  #[test]
  fn test_can_sub() {
    assert_eq!(
      Eval::val(5),
      Eval::sub(Eval::val(6), Eval::sub(Eval::val(2), Eval::val(1)))
    )
  }

  #[test]
  fn test_can_be_polymorphic_eval() {
    assert_eq!(Eval::val(20), add_10_and_10::<Eval>())
  }

  #[test]
  fn test_can_be_polymorphic_pretty() {
    assert_eq!("10 + 10", add_10_and_10::<PrettyPrint>())
  }

  #[test]
  fn test_can_pretty_print() {
    assert_eq!(
      "1 + 2",
      PrettyPrint::add(PrettyPrint::val(1), PrettyPrint::val(2))
    )
  }
}
