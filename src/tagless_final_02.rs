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
// It's called "final" because we define the EDSL by its semantics/interface of interpreters
//
// Overall, initial encoding makes it easy to add new interpreters,
// but expensive to add new language variants; whereas, final/tagless inverts that.

trait MulCalculator: Calculator {
  fn mul(a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

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
  type Repr;

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

impl MulCalculator for Eval {
  fn mul(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }
}

impl MulCalculator for PrettyPrint {
  fn mul(a: Self::Repr, b: Self::Repr) -> Self::Repr {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use crate::tagless_final_02::{Calculator, Eval, MulCalculator, PrettyPrint};

  #[test]
  fn test_can_mul() {
    assert_eq!(Eval::val(9), Eval::mul(Eval::val(3), Eval::val(3)))
  }

  #[test]
  fn test_can_mul_and_add_and_sub() {
    assert_eq!(
      Eval::val(36),
      Eval::mul(
        Eval::val(3),
        Eval::add(Eval::val(10), Eval::sub(Eval::val(3), Eval::val(1)))
      )
    )
  }

  #[test]
  fn test_can_pretty_print() {
    assert_eq!(
      "3 * 3",
      PrettyPrint::mul(PrettyPrint::val(3), PrettyPrint::val(3))
    )
  }
}
