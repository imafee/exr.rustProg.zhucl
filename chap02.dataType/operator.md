# 运算符

- 算术运算符
- 关系运算符
- 逻辑运算符
- 位运算符

## 算术运算符

加+、减-、乘\*、除/、求余%
可以对整型、浮点数进行运算，返回对应的数据类型的字面量。
字符串可以使用加+元算符进行连接运算，返回新的字符串。
注意点：rust 中没有自增++和自减--

## 关系运算符

<,<=,>,>=,==,!=
可以对布尔类型的值进行运算。

## 位运算符

对二进制数进行运算，返回对应的字符串类型的字面量。

- & 位与，1 和 1 得 1，否则 0
- | 位或，任意位为 1 得 1，否则 0
- ^ 异或，不相同时得 1，否则 0
- ! 位非，1 得 0，0 得 1
- << 左移，所有位左移指定位数，右边的位补 0
- \>> 右移，所有位右移指定位数，左边的位补 0