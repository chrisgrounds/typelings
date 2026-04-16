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
// It's called "final" because reasons

trait Calculator {
  type Repr;

  fn val(v: i32) -> Self::Repr;
  fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr;
  fn sub(a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

struct Eval;

struct PrettyPrint;

impl Calculator for Eval {
  type Repr;

  fn val(v: i32) -> Self {
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
  type Repr;

  fn val(v: i32) -> Self {
    todo!()
  }

  fn add(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }

  fn sub(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use crate::tagless_final_01::{Calculator, Eval, PrettyPrint};

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
      Eval::val(3),
      Eval::sub(Eval::val(6), Eval::sub(Eval::val(2), Eval::val(1)))
    )
  }

  #[test]
  fn test_can_pretty_print() {
    assert_eq!(
      "1 + 2",
      PrettyPrint::add(PrettyPrint::val(1), PrettyPrint::val(2))
    )
  }
}
