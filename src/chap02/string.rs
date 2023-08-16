pub fn delete (){
  let mut s = String::from("abcdefgh");

  // 删除并返回最后字符对象，some
  println!("{:?}",s.pop()); // some('h');
  println!("{}",s);// "abcdefg"
  
  // 删除并返回指定位置的字符，字面量
  println!("{:?}",s.remove(6)); // 'g'
  println!("{}",s);// "abcdef"
  
  // 截断：删除从指定位置开始至结尾的所有字符
  s.truncate(3);
  println!("{}",s);// "abc"
  
  // 空字符串字面量
  s.clear();
  println!("{}",s);// ""
}

pub fn replace(){
  let s = String::from("aaabbbcccaadd");
  let s1 = s.replace("aa","1"); // 匹配到的都替换
  let s2 = s.replacen("aa","2",1); // 只匹配1个

  println!("{}",s1);
  println!("{}",s2);
}

pub fn add(){
  let mut s = String::from("Hello,");
  s.push('R'); // 最后添加字符
  s.push_str("ust!"); // 最后添加字符串
  println!("{}",s); // "Hello,Rust"

  s.insert(6,' '); // 指定位置插入字符
  s.insert_str(6,"welcome to the"); // 指定位置插入字符串
  println!("{}",s); // "Hello,welcome to the Rust"
}

pub fn join(){
  let s1 = String::from("Hello");
  let s2 = String::from("Rust");
  let s3 = "World";
  let s = format!("{}-{}-{}",s1,s2,s3); // 格式化宏，参数可以是字符串对象、字符串字面量
  let mut ss = s1 + &s2 + s3;
  ss += "!";
  println!("{}",s);
  println!("{}",ss);
}

pub fn len(){
  let s1 = String::from("A");
  let s2 = String::from("AA");
  let s3 = String::from("一二三");
  println!("{},{},{}",s1.len(),s2.len(),s3.len()); // 1,2,9
}
pub fn char(){
  let s = String::from("123四五六");
  let bytes = s.bytes();
  let chars = s.chars();

  for b in bytes{
    print!("{},",b);
  }
  println!("\n");
  for c in chars{
    print!("{},",c);
  }
  println!("\n");
  //? 若想在最后一个成员之后打印一个\n，如何获悉
}