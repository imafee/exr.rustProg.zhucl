// 符合类型
pub fn tuple(){
  let tup1 = (1,); // 只有一个元素时，加上逗号
  let tup2 = (1,2,3);
  let tup8:(i8,f32,bool) = (-1,1.1,false); // 左右的类型和数量要匹配
  let tup9 = (1,(false,1.1),2); // tuple里面有tuple

  println!("{}",tup1.0); // 使用序号来访问
  println!("{},{},{}",tup2.0,tup2.1,tup2.2);

  let (x,y,z) = tup8; // 解构
  println!("{},{},{}",x,y,z);

  println!("{},{},{},{}",tup9.0,(tup9.1).0,(tup9.1).1,tup9.2);
}
pub fn array(){
  let arr1:[i32;5] = [1,2,3,4,5];
  let arr2:[i32;5] = [1;5]; // 长度为5
  let arr3 = [1,2,3,4,5];

  println!("{:?}",arr1);
  println!("{:?}",arr2);
  println!("{:?}",arr3);
  println!("arr3[0]:{},arr3[1]:{}",arr3[0],arr3[1]);
}
pub fn struct1(){

}
pub fn enum1(){
  #[derive(Debug)]
  enum Color {
    Red,
    Yellow,
    Blue,
  }
  let color = Color::Red;
  match color {
    Color::Red => println!("{:?}",Color::Red),
    Color::Yellow => println!("{:?}",Color::Yellow),
    Color::Blue => println!("{:?}",Color::Blue),
  }
}

pub fn enum2(){
  #[derive(Debug)]
  enum Color {
    Red(String),
    Yellow(String),
    Blue(String),
  }
  println!("{:?}",Color::Blue(String::from("blue")))
}