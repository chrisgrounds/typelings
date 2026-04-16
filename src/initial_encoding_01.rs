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
// It is "initial" because reasons
// It is "encoding" because each option is encoded as a type

#[derive(PartialEq, Eq, Debug)]
enum Calculator {
  Value(i32),
  Add(Box<Calculator>, Box<Calculator>),
  Sub(Box<Calculator>, Box<Calculator>),
}

fn eval(calculator: Calculator) -> i32 {
  todo!()
}

fn pretty_print(calculator: Calculator) -> String {
  todo!()
}

#[cfg(test)]
mod test {
  use crate::initial_encoding_01::{Calculator, eval, pretty_print};

  #[test]
  fn test_can_add() {
    assert_eq!(
      6,
      eval(Calculator::Add(
        Box::new(Calculator::Add(
          Box::new(Calculator::Value(1)),
          Box::new(Calculator::Value(2))
        ),),
        Box::new(Calculator::Value(3)),
      ))
    );
  }

  #[test]
  fn test_can_sub() {
    assert_eq!(
      1,
      eval(Calculator::Sub(
        Box::new(Calculator::Sub(
          Box::new(Calculator::Value(6)),
          Box::new(Calculator::Value(2))
        ),),
        Box::new(Calculator::Value(3)),
      ))
    );
  }

  #[test]
  fn test_can_pretty_print() {
    assert_eq!(
      "1 + 1",
      pretty_print(Calculator::Add(
        Box::new(Calculator::Value(1)),
        Box::new(Calculator::Value(1))
      ))
    )
  }
}
