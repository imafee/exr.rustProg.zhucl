# IEEE 754

IEEE 754 标准中对不同位数的浮点数表示格式的完整规定如下:

单精度浮点数(32 位):
1 位符号位
8 位指数位
23 位尾数位
双精度浮点数(64 位):
1 位符号位
11 位指数位
52 位尾数位
四精度浮点数(128 位):
1 位符号位
15 位指数位
112 位尾数位
八精度浮点数(256 位):
1 位符号位
19 位指数位
236 位尾数位
十六精度浮点数(512 位):
1 位符号位
20 位指数位
491 位尾数位

IEEE 754 标准在存储尾数时,小数点前的整数部分"1"是不需要显式存储的。原因如下:

- 尾数的二进制表示包含整数部分和小数部分。
  对于 1.xx 来说,整数部分恒为 1。
- 为了节省存储空间,IEEE 754 规定尾数存储时省略整数部分的 1。
- 整数部分的 1 被 “隐含” 在了指数部分。
- 指数存储时偏移了 127,就是为了将尾数的整数部分 1 转化为指数表示。
- 这样尾数存储时就只需要记录小数部分即可。
- 在解码时根据指数值,可以还原出整数部分的 1。

综上,IEEE 754 通过这种“隐含存储”的设计,避免了对整数部分 1 的重复存储, thus 节省了存储空间。这是尾数二进制表示的重要规定。

## 32 位单精度浮点数的存储算法

指数位的存储算法

```js
// 1. 计算指数值
function expBits(floatNum) {
  let exponent = Math.floor(Math.log10(Math.abs(floatNum)));

  // 2. 计算指数对应的二进制
  let exp = exponent + 127;
  let expBits = exp.toString(2);

  // 3. 补足8位指数位
  while (expBits.length < 8) {
    expBits = "0" + expBits;
  }

  return expBits;
}

// 5. 获得最终的指数位存储
console.log(expBits(12.34));
```

尾数的存储算法

```js
function mantissaBits(floatNum) {
  // 1. 获取小数部分
  let frac = floatNum % 1;

  // 2. 转换为二进制
  let fracBits = "";
  while (frac > 0) {
    if (frac >= 0.5) {
      fracBits += "1";
      frac -= 0.5;
    } else {
      fracBits += "0";
    }
    frac *= 2;
  }

  // 3. 补零到23位
  while (fracBits.length < 23) {
    fracBits += "0";
  }
  return fracBits;
}
```

整体的存储算法如下：

```js
let floatNum = 12.34;
let sign = floatNumber < 0 ? 1 : 0;
let exponentBits = expBits(floatNum);
let mantissaBits = mantBits(floatNum);
let bits = sign + exponentBits + mantissaBits;
console.log(bits); // 二进制表示
```
