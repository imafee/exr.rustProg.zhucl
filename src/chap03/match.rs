// match 模式匹配
fn main() {
  let age = 16;
  match age {
    0 => println!("you are a baby."),
    1..=2 => println!("you are a toddler."),
    3..=4 => println!("you are a preshooler."),
    5..=9 => println!("you are a schoolchild."),
    10..=11 => println!("you are a preteen."),
    12..=17 => println!("you are a teenager."),
    18..=100 => println!("you are a adult."),
    _ => (),
  }
}
