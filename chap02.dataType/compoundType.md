# compound data types (值类型为复合数据结构)

复合类型（Compound types）可以将多个值组合成一个类型。

- Tuple 元祖
- Array 数组
- struct 结构体
- enum 枚举

## Tuple 元祖类型

由一个或多个类型的元素组合而成：

- 字面量是(type1,type2,...)这样的形式
- 元素的类型可以不同
- 长度一旦定义不可更改
- 声明时一旦指定了类型与个数，那么等号右侧也要有相同的类型和个数
- 使用“元祖名.索引”来访问元素

## Array 数组类型

- 元素类型相同
- 长度固定

## struct 结构体类型

以 kv 对组合的结构

```rust
// 对象结构
struct 结构体名字 {
  name: type,
}
// 元祖结构
struct 结构体名称 (type1,type2,...)
// 单元结构
struct 结构体名称;
```

## enum 枚举类型

有限个数的值的结构

- 无参数
- 有参数
  以 枚举名::枚举值 来访问

无参数

```rust
#[derive(Debug)]
enum Color {
  Red,
  Yellow,
  Blue
}
let color = Color::Red;
println!("{:?}",color); // Red
```

有参数

```rust
#[derive(Debug)]
enum Color {
  Red(String),
  Yellow(String),
  Blue(String)
}
let color = Color::Red(String::from("红色啊"));
println!("{:?}",color); // Red("红色啊")
```
