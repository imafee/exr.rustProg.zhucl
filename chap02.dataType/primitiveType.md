# Primitive types

TOC

参考：
https://doc.rust-lang.org/book/ch03-02-data-types.html
https://doc.rust-lang.org/reference/types.html

- Numeric （数值）
  - Integer 整数型
  - Floating-Point 浮点型
- Boolean 布尔型
- Char 字符型
- Never 任意值类型，使用!符号表示

## Integer 整型

- 无符号整数(unsigned integer)，简写为 u
- 有符号整数(signed integer),简写为 i

整型数据的可存储长度

<!--prettier-ignore-->
|Byte长度(n)|bit长度(8*n)|有符号表示|无符号表示|
---|---|---|---
1|8|i8|u8
2|16|i16|u16
4|32|i32(rust默认)|u32
8|64|i64|u64
16|128|i128|u128
arch|arch|isize|usize

注

```doc
isize 和 usize 表示：系统是多少位，bit 长度就是多少。
比如 64bit 系统中，`let num :isize = 1`,那么 num 就是 i64 的值类型
```

链接：

- [计算机信息的表示单位和存储单位](../doc/computerDataProcessing.md)
- [Q: 为什么 rust 在设计值类型时采用跳跃式的字节数呢](../doc/QA.md#为什么-rust-在设计值类型时采用跳跃式的字节数呢)

字面量的多种写法

```rust
// 显式声明值类型
let integer0:usize = 1;
let integer1:u32 = 1;
let integer2 = 1u32; // 后缀指明类型
// 隐式声明值类型
let integer3 = 1; // i32，默认
// 数值的可读性写法
let integer4 = 10_000; // _叫做可读性分隔符，编译器最终会把它去掉，实际数值是10000
// 进制数值的表示法
let integer_binary:u32 = 0b10; // 2进制
let integer_octal:u32 = 0o10; // 8进制
let integer_hexadecimal:u32 = 0x10; // 16进制
```

值类型对应的数值范围，公式如下
(设字节长度为 n。则 bit 长度为 8n，计作 m)

- 无符号：[0,2^m-1]
- 有符号：[-2^(m-1),2^(m-1)-1]

因此值类型对应的值范围为：

<!--prettier-ignore-->
|Byte长度(n)|bit 位长度(8*n)|有符号的值范围|无符号的值范围|
---|---|---|---
1|8|[-128,127]|[0,255]
2|16|[-2^15,2^15-1]|[0,2^16-1]
4|32|[-2^31,2^31-1]|[0,2^32-1]
8|64|[-2^63,2^63-1]|[0,2^64-1]
16|128|[-2^127,2^127-1]|[0,2^128-1]

## 浮点数型

遵循 IEEE 754 标准,浮点数的最高位是符号位,0 表示正数,1 表示负数。剩余位用于存储指数和小数位数。
按照信息的可存储长度，浮点型被细分设计为：

<!--prettier-ignore-->
|Byte长度(n)|bit位长度(8*n)|有符号表示|备注|
---|---|---|---
4|32|f32|小数点后的位数[6,14]
8|64|f64(rust默认)|小数点后的位数[15,?]

f32 称为单精度浮点数，f64 称为双精度浮点数

字面量的写法

```rust
let float1:f32 = 1.1;
let float2 = 1.1f32;
let float3 = 1.1; // f64
let float4 = 11_000.222_333_444
```

?了解：IEEE 754 的具体算法

## 布尔类型

字面量的写法

```rust
let t: bool = true; // 显示类型声明
let f = false; // 隐式类型声明
```

## 字符类型

rust 使用 UTF-8 作为语言的编码。
字符类型代表的是一个 Unicode 标量值(Unicode Scalar Value)，包括数字、字母、Unicode 字符和其他特殊字符。
每个字符占 4 个 Byte。
字符类型的字面量表示由单引号来定义。

```rust
let c = 'z';
let cc = '⚽︎'

```

## Never 类型

```rust
let x: ! = abc!();
// Can be coerced into any type.
let y: i32 = x;
```

## ?范围类型?

官网文档中并没有发现此类数据类型！
字面量表示的是一个整数序列，有 2 种形式：

- 左闭右开，比如(1..5)
- 全闭,比如(1..=5)

```rust
fn main(){
  /// 2种整数序列
  print!("(1..3):");
  for i in 1..3{
    print!("{}",i);
  } // 1 2
  print!("(1..=3):");
  for i in 1..=3{
    print!("{}",i);
  } // 1 2 3

  /// 方法演示
  print!("(1..=3).rev:");
  for i in (1..=3).rev(){
    print!("{}",i);
  } // 3 2 1
  let sum:u32 = (1..=3).sum();
  println!("1+2+3={}",sum); // 6
}
```
