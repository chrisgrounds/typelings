// Let's define an "embedded domain-specific-language".
// This is a DSL that doesn't require it's own parser/type-checking
// as it is embedded in the langauge it's written in.
//
// We will want to perform multiple different actions on this DSL
// such as performing the calculation, pretty-printing out each step,
// printing out the result, etc
//
// We will define this DSL as an "initial-encoding".
// An initial-encoding represents data as a data-type/AST
// and adds on operations in the form of functions
//
// It's called "initial" because
// It's called "encoding" because each option is encoded as a type
#[derive(PartialEq, Eq, Debug)]
enum Calculator {
  Value(i32),
  Add(Box<Calculator>, Box<Calculator>),
  Sub(Box<Calculator>, Box<Calculator>),
  // TODO: Add Mul(a, b) to encode multiply
}

fn eval(calculator: Calculator) -> i32 {
  todo!()
}

fn pretty_print(calculator: Calculator) -> String {
  todo!()
}

#[cfg(test)]
mod test {
  use crate::initial_encoding_02::{Calculator, eval};

  #[test]
  fn test_can_mul() {
    assert_eq!(
      9,
      eval(Calculator::Mul(
        Box::new(Calculator::Mul(
          Box::new(Calculator::Value(1)),
          Box::new(Calculator::Value(3))
        ),),
        Box::new(Calculator::Value(3)),
      ))
    );
  }
}
