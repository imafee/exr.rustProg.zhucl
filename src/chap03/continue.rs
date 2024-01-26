// continue,break

fn main() {
  for i in 0..10 {
    if i == 0 || i == 4 {
      continue;
    }
    if i == 6 {
      break;
    }
    println!("i:{}", i);
  }
}
// 1,2,3,5
