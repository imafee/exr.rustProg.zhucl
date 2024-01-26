// loop循环

fn main() {
  let mut count = 0;
  let counter = loop {
    count += 1;
    let counter = count * 2;
    println!("count:{},counter:{}", count, counter);

    if count == 10 {
      break counter;
    }
  };
}
