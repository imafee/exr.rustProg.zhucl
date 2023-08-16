# 字符串

字符串是一种特殊的容器类型，由零个到多个字符组成的有限序列。
不同于其他容器类型关注于容器内的元素，字符串被作为一个整体来关注和使用，因此单独拎出来讲。

分为 2 种：

- 固定长度的字符串字面量 str （rust 内置）
- 可变长度的字符串对象 String

## 创建字符串

字符串字面量 &str（通常以引用的形式出现） 是字符的集合，代表的是不可变的 UTF-8 编码的字符串的引用。
创建后无法再继续修改其内容。

```rust
// 第一种: 使用双引号创建字符串字面量
let s1 = "Hello";
// 第二种:
let str = String::from("Hello"); // 字符串对象
let s2 = str.as_str(); // 转换为字符串字面量
```

字符串对象 String 是 Rust 标准库提供的，创建后可以修改其内容。
String 类型的本质是一个字段为 Vec<u8>类型的结构体，它把字符内容存放在堆上，组成如下：

- 指向堆上字节序列的指针(as_ptr 方法)
- 记录堆上字节序列的长度(len 方法)
- 堆分配的容量(capacity 方法)

```rust
// 第一种
let mut s1 = String::new(); // 空字符串对象
// 第二种: 根据字面量来创建字符串对象
let s2 = String::from("Hello");
// 第三种: 将字面量值转换为字符串对象
let str = "Hello";
let s3 = str.to_string();
```

## 字符串的修改

字符串的常见操作：

- 增：push 追加，insert 插入
- 删：pop 删除最后单字符、remove 删除指定位置的单字符、truncate 删除指定范围内的多个字符、clear 删除所有字符
- 改：replace、replacen 替换
- 连接："+"操作符只能在字面量种使用，通常使用 format！格式化宏

push 追加、insert 插入,见`/src/chap02/string.rs`

pop 删除最后单字符、remove 删除指定位置的单字符、
truncate 删除指定范围内的多个字符、clear 删除所有字符.`见/src/chap02/string.rs`

replace、replacen,见`/src/chap02/string.rs`

连接：字面量的连接、format!宏的使用.`见/src/chap02/string.rs`

## 字符串的访问

- 字符串是 UTF-8 编码的字节序列，所以不能直接使用索引来访问字符
- 字符串操作可以分为按字节处理、按字符操作
  - bytes 方法返回按字节迭代的迭代器
  - chars 方法返回按字符迭代的迭代器

使用 len 方法获取字符串的长度
见`/src/chap02/string.rs`的 len

使用迭代器访问字符串的字符
见`/src/chap02/string.rs`的 char
