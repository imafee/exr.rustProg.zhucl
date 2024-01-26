// 模式匹配的语法糖

fn main() {
  // if let
  fn match_value(value: Option<i32>) {
    match value {
      Some(7) => println!("seven"),
      _ => (),
    }
  }
  fn if_let_value(value: Option<i32>) {
    if let Some(8) = value {
      println!("eight")
    }
  }

  match_value(Some(7));
  if_let_value(Some(8));

  // while let
  fn match_vec() {
    let mut vec = vec![1, 2, 3, 4, 5];
    loop {
      match vec.pop() {
        Some(value) => println!("{}", value),
        None => break,
      }
    }
  }
  fn while_let_vec() {
    let mut vec = vec![1, 2, 3, 4, 5];
    while let Some(value) = vec.pop() {
      println!("{}", value);
    }
  }

  match_vec();
  while_let_vec();
}
