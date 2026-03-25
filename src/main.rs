// =============================================================================
// Rust 系统性学习教程 - 完整版
// 循序渐进的 Rust 标准库知识点演示
// 每个知识点都有详细的注释
// =============================================================================

use std::collections::HashMap;

// =============================================================================
// 第一章：变量与常量
// =============================================================================

mod chapter01_variables {

    /// 知识点 1.1: 变量基本声明
    ///
    /// Rust 中变量默认是不可变的（immutable），这是 Rust 的核心安全特性之一。
    ///
    /// ## 为什么默认不可变？
    /// 1. **数据竞争预防**：当数据不可变时，多个线程同时访问也不会出现数据竞争
    /// 2. **并发安全**：编译器可以在编译期检查并发问题
    /// 3. **思维清晰**：代码意图更明确，减少意外的修改
    /// 4. **编译器优化**：不可变数据可以让编译器进行更多优化
    ///
    /// ## 对比其他语言
    /// - C/C++: 变量默认可变，需要 const 限制
    /// - JavaScript: 变量默认可变，需要 const 限制
    /// - Rust: 变量默认不可变，需要 mut 允许修改
    ///
    /// ## 示例说明
    /// 下面的 x 一旦赋值就不能再修改，这确保了数据的安全性
    #[allow(dead_code)]
    pub fn basic_variable() -> i32 {
        let x = 5;
        x
    }

    /// 知识点 1.2: 可变变量
    ///
    /// 使用 `mut` 关键字可以让变量变为可变的。
    ///
    /// ## 语法说明
    /// - `let mut x = 5;` - 声明一个可变变量 x，初始值为 5
    /// - `x = 6;` - 可以对 x 进行重新赋值
    ///
    /// ## 何时使用 mut？
    /// - 当变量值需要被修改时
    /// - 计数器、累加器等
    /// - 需要更新状态的场景
    ///
    /// ## 最佳实践
    /// 先用不可变变量（let），只有确实需要修改时才加 mut。
    /// 这样可以让编译器帮助我们发现潜在的错误。
    #[allow(dead_code, unused_assignments)]
    pub fn mutable_variable() -> i32 {
        let mut x = 5;
        x = 6;
        x
    }

    /// 知识点 1.3: 常量
    ///
    /// 常量使用 `const` 关键字，必须显式标注类型。
    ///
    /// ## 常量 vs 不可变变量
    /// | 特性 | const | let |
    /// |------|-------|-----|
    /// | 编译时确定 | ✓ | ✓ |
    /// | 运行时确定 | ✗ | ✓ |
    /// | 必须标注类型 | ✓ | ✗ |
    /// | 可用于全局 | ✓ | ✗ |
    /// | 可作为数组大小 | ✓ | ✗ |
    ///
    /// ## 命名规范
    /// 常量通常使用全大写字母加下划线：`MAX_SIZE`, `PI`, `BUFFER_SIZE`
    ///
    /// ## 注意事项
    /// - 常量必须在编译时就能确定值，不能使用运行时的计算结果
    /// - 常量可以在任何作用域中声明，包括全局作用域
    /// - 常量没有内存地址（在编译时被内联）
    #[allow(dead_code)]
    pub fn constant_demo() -> u32 {
        const MAX_SIZE: u32 = 100;
        const PI: f64 = 3.14159;
        println!("常量: MAX_SIZE={}, PI={}", MAX_SIZE, PI);
        MAX_SIZE
    }

    /// 知识点 1.4: 变量遮蔽（Shadowing）
    ///
    /// 可以在同一个作用域内用 `let` 声明同名变量，后面的变量会"遮蔽"前面的。
    ///
    /// ## 核心概念
    /// 变量遮蔽不是修改了原有变量，而是创建了一个全新的同名变量。
    /// 这在 Rust 中是合法的，因为 `let` 本质上是在声明一个新的绑定。
    ///
    /// ## 重要特性：类型可以改变！
    /// 这是 Rust 独有的特性，第一次是 i32，第二次可以是 &str。
    /// 这是因为我们实际上创建了不同类型的新变量，而不是修改原变量。
    ///
    /// ## 使用场景
    /// 1. **值转换**：将一种类型转换为另一种类型时很有用
    ///    ```rust
    ///    let s = "   ";  // &str
    ///    let s = s.len();  // usize
    ///    ```
    /// 2. **增量构建**：在复杂计算中逐步构建值
    /// 3. **代码清晰**：在同一个作用域内处理不同阶段的数据
    ///
    /// ## vs mut
    /// - `let mut` = 修改变量的值
    /// - `let` (shadowing) = 创建新变量（值和类型都可以改变）
    pub fn shadowing() -> usize {
        let x = 5;
        println!("原始 x = {}", x);

        let x = x + 1; // 遮蔽：创建新变量 x，值为 6
        println!("遮蔽后 x = {}", x);

        let x = "hello"; // 再次遮蔽：类型从 i32 变成 &str
        println!("类型转换后 x = {}", x);
        x.len()
    }
}

// =============================================================================
// 第二章：基本数据类型
// =============================================================================

mod chapter02_types {
    /// 知识点 2.1: 整数类型
    ///
    /// Rust 提供了多种整数类型，用于不同的场景和内存管理需求。
    ///
    /// ## 有符号 vs 无符号
    /// - **有符号 (i开头)**：可以表示负数，范围是 -(2^(n-1)) 到 2^(n-1) - 1
    /// - **无符号 (u开头)**：只能表示非负数，范围是 0 到 2^n - 1
    ///
    /// ## 类型选择建议
    /// - `i32`：默认选择，大多数场景下效率最高
    /// - `u32`：当需要非负数且值可能超过 i32 范围时
    /// - `i64`/`u64`：大数值计算，或与外部系统交互时
    /// - `isize`/`usize`：用于索引集合（数组、Vec等），长度依赖于运行平台
    ///   - 64位系统上是 64 位，32位系统上是 32 位
    ///
    /// ## 内存占用
    /// - i8/u8: 1 字节 (8位)
    /// - i16/u16: 2 字节 (16位)
    /// - i32/u32: 4 字节 (32位) ← 最常用
    /// - i64/u64: 8 字节 (64位)
    /// - i128/u128: 16 字节 (128位) - 极大数计算
    pub fn integer_types() -> (i32, u32, i64, u64) {
        let a: i32 = 42;
        let b: u32 = 100;
        let c: i64 = -123;
        let d: u64 = 999;
        println!("整数示例: a={}, b={}, c={}, d={}", a, b, c, d);
        (a, b, c, d)
    }

    /// 知识点 2.2: 整数字面量
    ///
    /// Rust 支持多种进制的整数字面量表示法，方便阅读和编程。
    ///
    /// ## 语法格式
    /// | 前缀 | 进制 | 示例 | 十进制值 |
    /// |------|------|------|---------|
    /// | (无) | 十进制 | 98_222 | 98222 |
    /// | 0x | 十六进制 | 0xff | 255 |
    /// | 0o | 八进制 | 0o77 | 63 |
    /// | 0b | 二进制 | 0b1111_0000 | 240 |
    ///
    /// ## 下划线分隔符
    /// - 可以使用 `_` 来提高大数的可读性
    /// - `_` 会被编译器忽略，效果与不加一样
    /// - 常见用法：千位分隔、字节分组
    ///
    /// ## 字节字面量
    /// - `b'A'` 表示 ASCII 字符 'A' 的字节值 (65)
    /// - 必须是单个字符，且是 ASCII 范围 (0-127)
    /// - 返回类型是 u8
    ///
    /// ## 类型后缀
    /// 可以为字面量指定类型：`42u32`, `100i64`, `255u8`
    #[allow(dead_code)]
    pub fn integer_literals() -> (u32, u32, u32, u32, u8) {
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        println!("十进制: {}, 十六进制: {}, 八进制: {}, 二进制: {}, 字节: {}",
                 decimal, hex, octal, binary, byte);
        (decimal, hex, octal, binary, byte)
    }

    /// 知识点 2.3: 浮点类型
    ///
    /// Rust 提供两种浮点类型：f32（单精度）和 f64（双精度）。
    ///
    /// ## f32 vs f64
    /// - **f32**: 32位，单精度，精度约 7 位小数
    /// - **f64**: 64位，双精度，精度约 15 位小数 ← 默认选择
    ///
    /// ## 为什么默认用 f64？
    /// 1. 在现代 CPU 上，f64 和 f32 性能几乎相同
    /// 2. f64 提供更高的精度，避免舍入误差
    /// 3. 与其他语言（如 Python、JavaScript）的默认行为一致
    ///
    /// ## 浮点数的特殊性
    /// - 遵循 IEEE 754 标准
    /// - 无法精确表示所有小数（如 0.1 + 0.2 ≠ 0.3）
    /// - 存在正负无穷大（inf）和非数值（NaN）
    /// - 整数除法会得到浮点数：`5 / 2 = 2.5`
    ///
    /// ## 数值计算
    /// 浮点数支持 +, -, *, / 等算术运算
    pub fn floating_types() -> (f64, f64, f64) {
        let x = 2.0;
        let y: f64 = 3.0;
        let z = -1.5;
        let sum = x + z;
        let product = x * y;
        println!("x={}, y={}, z={}, sum={}, product={}", x, y, z, sum, product);
        (x, y, z)
    }

    /// 知识点 2.4: 布尔类型
    ///
    /// 布尔类型只有两个值：`true` 和 `false`，用于条件判断和逻辑运算。
    ///
    /// ## 基本用法
    /// - 条件语句：`if`, `while`, `match` 等
    /// - 逻辑运算：`&&` (与), `||` (或), `!` (非)
    /// - 比较运算：`==`, `!=`, `<`, `>`, `<=`, `>=`
    ///
    /// ## 内存表示
    /// - 实际占用 1 个字节（不是 1 位）
    /// - 这是因为内存寻址的最小单位是字节
    ///
    /// ## 类型转换
    /// Rust 不支持布尔类型与其他类型之间的隐式转换。
    /// 必须使用 `as` 显式转换：`true as i32 = 1`, `false as i32 = 0`
    ///
    /// ## 最佳实践
    /// - 使用有意义的布尔变量名：`is_valid`, `has_permission`, `is_empty`
    /// - 避免使用 `if condition == true`，直接用 `if condition`
    /// - 避免使用 `if condition == false`，应该用 `if !condition`
    #[allow(dead_code)]
    pub fn boolean_type() -> (bool, bool) {
        let t = true;
        let f: bool = false;
        println!("t={}, f={}", t, f);
        (t, f)
    }

    /// 知识点 2.5: 字符类型
    ///
    /// Rust 的 `char` 类型表示一个 Unicode 标量值（Unicode Scalar Value）。
    ///
    /// ## 重要特性
    /// - **不是 ASCII**：Rust 的 char 是完整的 Unicode 字符
    /// - **4 字节**：实际上是 32 位，可以表示所有 Unicode 字符
    /// - **能表示中文、日文、Emoji 等**
    ///
    /// ## Unicode 范围
    /// - 0x0000 到 0xD7FF
    /// - 0xE000 到 0x10FFFF
    /// 不包括代理对（surrogate pairs）：0xD800 到 0xDFFF
    ///
    /// ## 与其他语言的对比
    /// - C/C++ 的 char：通常是 1 字节，只能表示 ASCII
    /// - Java 的 char：2 字节，只能表示 UTF-16 单元
    /// - Rust 的 char：4 字节，表示完整的 Unicode 标量值
    ///
    /// ## 使用注意
    /// - 字符串中的字符可能是多字节的，所以 `len()` 返回字节数而非字符数
    /// - 要获取真正的字符数，需要使用 `.chars().count()`
    pub fn char_type() -> (char, char, char) {
        let c1 = 'a';
        let c2: char = '中';
        let c3 = '😀';
        println!("字符: c1='{}', c2='{}', c3='{}'", c1, c2, c3);
        (c1, c2, c3)
    }

    /// 知识点 2.6: 类型转换
    ///
    /// Rust 使用 `as` 关键字进行显式的类型转换（Type Casting）。
    ///
    /// ## 基本语法
    /// `目标类型 as 源值` 或 `源值 as 目标类型`
    ///
    /// ## 转换规则
    /// 1. **整数之间**：可以相互转换，溢出时采用 wrapping 方式
    /// 2. **整数转浮点**：完美转换
    /// 3. **浮点转整数**：小数部分被截断（不是四舍五入！）
    /// 4. **字符转整数**：得到字符的 Unicode 码点
    ///
    /// ## 危险行为
    /// - 浮点数转整数时，向零取整：`3.99 as i32 = 3`, (-3.99 as i32 = -3)
    /// - 大数转小数时，可能溢出为 inf
    /// - 无符号转有符号时，可能产生负数（按补码解释）
    ///
    /// ## 安全转换方法
    /// 使用 `TryFrom` 和 `TryInto` trait 可以进行安全的转换：
    /// ```rust
    /// let num: i32 = "42".parse().ok().unwrap();
    /// ```
    ///
    /// ## 注意事项
    /// - `as` 不会进行边界检查，可能导致未定义行为
    /// - 优先使用安全的转换方法
    #[allow(dead_code)]
    pub fn type_casting() -> (i32, f64, f64) {
        let a: i32 = 10;
        let b: i64 = a as i64;  // i32 -> i64，小转大，安全
        let c: f64 = a as f64;  // i32 -> f64，整数转浮点
        let d: i32 = 3.99 as i32; // 浮点 -> i32，截断小数部分
        println!("a={}, b={}, c={}, d={}", a, b, c, d);
        (d as i32, c, c)
    }
}

// =============================================================================
// 第三章：运算符
// =============================================================================

mod chapter03_operators {


    /// 知识点 3.1: 算术运算符
    ///
    /// Rust 支持标准的算术运算符，用于数值计算。
    ///
    /// ## 运算符列表
    /// | 运算符 | 名称 | 示例 | 结果 |
    /// |--------|------|------|------|
    /// | + | 加法 | 10 + 3 | 13 |
    /// | - | 减法 | 10 - 3 | 7 |
    /// | * | 乘法 | 10 * 3 | 30 |
    /// | / | 除法 | 10 / 3 | 3 |
    /// | % | 取余 | 10 % 3 | 1 |
    ///
    /// ## 重要特性
    /// 1. **整数除法**：向零取整，10/3=3，(-10)/3=-3
    /// 2. **溢出处理**：Debug 模式下会 panic，Release 模式下会 wrap
    ///    - 使用 `checked_add`, `overflowing_add`, `wrapping_add` 处理溢出
    /// 3. **浮点除法**：返回浮点数，10.0/3.0=3.333...
    ///
    /// ## 重载
    /// 这些运算符可以通过实现 `Add`, `Sub`, `Mul`, `Div`, `Rem` trait 来重载
    /// 使得自定义类型也能使用这些运算符
    #[allow(dead_code)]
    pub fn arithmetic_operators() -> (i32, i32, i32, i32, i32) {
        let a = 10;
        let b = 3;
        let sum = a + b;
        let diff = a - b;
        let product = a * b;
        let quotient = a / b;
        let remainder = a % b;
        println!("{} + {} = {}", a, b, sum);
        (sum, diff, product, quotient, remainder)
    }

    /// 知识点 3.2: 关系运算符
    ///
    /// 关系运算符比较两个值，返回布尔类型的结果。
    ///
    /// ## 运算符列表
    /// | 运算符 | 名称 | 示例 | 含义 |
    /// |--------|------|------|------|
    /// | == | 等于 | a == b | a 和 b 相等 |
    /// | != | 不等于 | a != b | a 和 b 不相等 |
    /// | < | 小于 | a < b | a 小于 b |
    /// | > | 大于 | a > b | a 大于 b |
    /// | <= | 小于等于 | a <= b | a 小于或等于 b |
    /// | >= | 大于等于 | a >= b | a 大于或等于 b |
    ///
    /// ## 特点
    /// - 所有关系运算符都返回 `bool` 类型
    /// - 可以用于任何实现了 `PartialOrd` trait 的类型
    /// - 支持链式调用：如 `0 < x && x < 10` 可写作 `0 < x < 10`（在某些语言中）
    ///   但 Rust 不支持这种写法，需要分别写
    ///
    /// ## 浮点数的比较
    /// 浮点数实现了 `PartialOrd`，但注意：
    /// - NaN 比较结果永远为 false：即使 `NaN < 0` 也是 false
    /// - 使用 `a.partial_cmp(&b)` 来安全处理 NaN
    #[allow(dead_code)]
    pub fn relational_operators() -> Vec<bool> {
        let a = 5;
        let b = 10;
        vec![a == b, a != b, a < b, a > b, a <= b, a >= b]
    }

    /// 知识点 3.3: 逻辑运算符
    ///
    /// 逻辑运算符用于组合布尔值，进行布尔代数运算。
    ///
    /// ## 运算符列表
    /// | 运算符 | 名称 | 短路行为 | 说明 |
    /// |--------|------|----------|------|
    /// | && | 逻辑与 | 是 | 两者都为 true 时为 true |
    /// | \|\| | 逻辑或 | 是 | 任意一个为 true 时为 true |
    /// | ! | 逻辑非 | 否 | 取反，true 变 false |
    ///
    /// ## 短路求值（Short-circuit Evaluation）
    /// - `&&`：如果左边为 false，右边不计算，直接返回 false
    /// - `||`：如果左边为 true，右边不计算，直接返回 true
    /// 这可以用于条件执行和避免不必要的计算
    ///
    /// ## 示例
    /// ```rust
    /// // 避免除零错误
    /// if denominator != 0 && numerator / denominator > 0 { ... }
    ///
    /// // 使用 || 的默认值
    /// let value = option.unwrap_or(default);
    /// ```
    #[allow(dead_code)]
    pub fn logical_operators() -> (bool, bool, bool) {
        let a = true;
        let b = false;
        (a && b, a || b, !b)
    }

    /// 知识点 3.4: 位运算符
    ///
    /// 位运算符直接操作数据的二进制位，常用于底层编程和位掩码。
    ///
    /// ## 运算符列表
    /// | 运算符 | 名称 | 示例 | 结果 (二进制) | 说明 |
    /// |--------|------|------|---------------|------|
    /// | & | 按位与 | 1100 & 1010 | 1000 | 两位都为1时为1 |
    /// | \| | 按位或 | 1100 \| 1010 | 1110 | 任一位为1时为1 |
    /// | ^ | 按位异或 | 1100 ^ 1010 | 0110 | 两位不同时为1 |
    /// | << | 左移 | 1100 << 2 | 110000 | 右边补0 |
    /// | >> | 右移 | 1100 >> 2 | 11 | 取决于符号位 |
    ///
    /// ## 常见用途
    /// 1. **位掩码**：设置或清除特定位
    ///    ```rust
    ///    const READ: u8 = 0b001;  // 1
    ///    const WRITE: u8 = 0b010; // 2
    ///    let flags = READ | WRITE; // 3
    ///    ```
    /// 2. **乘除法**：左移相当于乘以2，右移相当于除以2（整数）
    /// 3. **加密**：XOR 加密
    /// 4. **快速判断**：判断奇偶 (n & 1)
    ///
    /// ## 有符号数的位移
    /// - 左移 `<<`：始终是逻辑左移，右边补0
    /// - 右移 `>>`：算术右移，负数补1，正数补0
    pub fn bitwise_operators() -> (u8, u8, u8, u8, u8) {
        let a: u8 = 0b1100;
        let b: u8 = 0b1010;
        let and = a & b;
        let or = a | b;
        let xor = a ^ b;
        let shift_left = a << 2;
        let shift_right = a >> 2;
        println!("a & b = {:04b} ({})", and, and);
        (and, or, xor, shift_left, shift_right)
    }
}

// =============================================================================
// 第四章：控制流
// =============================================================================

mod chapter04_control_flow {


    /// 知识点 4.1: if-else 条件分支
    ///
    /// Rust 的 `if-else` 是表达式，可以返回值。
    ///
    /// ## 基本语法
    /// ```rust
    /// if 条件 {
    ///     // 条件为 true 时执行
    /// } else {
    ///     // 条件为 false 时执行
    /// }
    /// ```
    ///
    /// ## 重要特性
    /// 1. **条件必须是 bool**：Rust 不会自动转换其他类型为布尔值
    ///    - 这是安全的，可以避免隐式的"真/假"判断歧义
    ///    - 错误示例：`if x { }`（x 不是 bool 会报错）
    ///    - 正确示例：`if x != 0 { }`
    ///
    /// 2. **if 是表达式**：可以赋值给变量！
    ///    ```rust
    ///    let result = if condition { "yes" } else { "no" };
    ///    ```
    ///    所有分支必须返回相同类型
    ///
    /// 3. **else if 链**：可以连续使用多个 else if
    ///    - 短路求值：从上到下依次判断
    ///    - 找到第一个满足条件的后停止
    pub fn if_else_demo() {
        let num = 5;
        if num > 0 {
            println!("num 是正数");
        } else {
            println!("num 不是正数");
        }

        let result = if num > 10 {
            "大于10"
        } else if num > 5 {
            "大于5且小于等于10"
        } else {
            "小于等于5"
        };
        println!("result = {}", result);
    }

    /// 知识点 4.2: match 模式匹配
    ///
    /// `match` 是 Rust 最强大的控制流结构，类似于其他语言的 switch，
    /// 但功能更强大、更安全。
    ///
    /// ## 基本语法
    /// ```rust
    /// match 值 {
    ///     模式1 => 表达式,
    ///     模式2 => 表达式,
    ///     _ => 默认表达式,  // 匹配所有情况
    /// }
    /// ```
    ///
    /// ## 重要特性
    /// 1. **必须穷尽所有可能**：编译器会检查是否覆盖了所有情况
    ///    - 可以使用 `_` 作为通配符处理剩余情况
    ///
    /// 2. **模式可以是字面值**：`1`, `2`, `3`
    ///    - 可以用 `|` 组合多个值：`6 | 7`
    ///
    /// 3. **返回值**：match 也是表达式，可以返回值
    ///
    /// 4. **解构功能**：可以匹配复杂数据结构
    ///
    /// ## vs switch
    /// | 特性 | match | switch |
    /// |------|-------|--------|
    /// | 穷尽检查 | 强制 | 可选 |
    /// | 模式复杂 | 支持 | 不支持 |
    /// | 返回值 | 是 | 否(一般) |
    pub fn match_demo() -> &'static str {
        let day = 3;
        let day_name = match day {
            1 => "星期一",
            2 => "星期二",
            3 => "星期三",
            4 => "星期四",
            5 => "星期五",
            6 | 7 => "周末",  // 用 | 连接多个值
            _ => "未知",       // 通配符，匹配所有其他情况
        };
        println!("day {} 是 {}", day, day_name);
        day_name
    }

    /// 知识点 4.3: loop 循环
    ///
    /// `loop` 是 Rust 中最基础的循环关键字，表示无限循环。
    /// 它与其他语言的 while true 或 for(;;) 类似。
    ///
    /// ## 基本语法
    /// ```rust
    /// loop {
    ///     // 无限循环，直到 break
    /// }
    /// ```
    ///
    /// ## 重要特性
    /// 1. **break 带返回值**：可以在 break 时返回一个值
    ///    ```rust
    ///    let result = loop {
    ///        if condition {
    ///            break value;  // 跳出循环并返回 value
    ///        }
    ///    };
    ///    ```
    ///
    /// 2. **带标签的 break**：可以跳出多层循环
    ///    ```rust
    ///    'outer: loop {
    ///        loop {
    ///            break 'outer;  // 直接跳出外层循环
    ///        }
    ///    }
    ///    ```
    ///
    /// 3. **continue**：跳过本次循环，继续下一次
    ///
    /// ## 典型用途
    /// - 重试机制：loop { ... try! ... break on success }
    /// - 服务器主循环
    /// - 需要明确退出的复杂逻辑
    pub fn loop_demo() -> i32 {
        let mut count = 0;
        // loop 表达式会返回 break 的值
        let result = loop {
            count += 1;
            if count == 5 {
                break count * 2;  // 跳出循环并返回值
            }
        };
        println!("result = {}", result);
        result
    }

    /// 知识点 4.4: for 循环
    ///
    /// Rust 的 `for` 循环用于遍历集合（数组、Vec、Range、迭代器等）。
    ///
    /// ## Range 语法
    /// - `1..5`：不含 5，生成 1, 2, 3, 4
    /// - `1..=5`：含 5，生成 1, 2, 3, 4, 5
    ///
    /// ## 重要特性
    /// 1. **安全遍历**：Rust 的 for 循环会自动获取集合的不可变引用
    ///    - 不会发生迭代过程中修改集合的错误
    ///
    /// 2. **enumerate()**：获取索引和值的元组
    ///    ```rust
    ///    for (i, val) in collection.iter().enumerate() { }
    ///    ```
    ///
    /// 3. **iter() vs into_iter() vs iter_mut()**
    ///    - `iter()`：获取不可变引用
    ///    - `iter_mut()`：获取可变引用
    ///    - `into_iter()`：获取所有权（消耗集合）
    ///
    /// 4. **反向遍历**：使用 `.rev()`
    ///    ```rust
    ///    for i in (1..=5).rev() { }
    ///    ```
    ///
    /// ## vs 其他语言
    /// Rust 的 for 更像 Python 的 for-in，而不像 C 的 for
    /// 不需要手动管理索引和边界
    pub fn for_demo() -> i32 {
        // Range: 1 到 5（不含 5），所以是 1,2,3,4
        let sum: i32 = (1..=5).sum();  // 1..=5 含 5
        println!("1 到 5 的和 = {}", sum);

        let arr = [10, 20, 30];
        // enumerate() 返回 (索引, 值) 的元组
        for (i, val) in arr.iter().enumerate() {
            println!("arr[{}] = {}", i, val);
        }
        sum
    }

    /// 知识点 4.5: while 循环
    ///
    /// `while` 循环在条件为 true 时重复执行代码块。
    ///
    /// ## 基本语法
    /// ```rust
    /// while 条件 {
    ///     // 循环体
    /// }
    /// ```
    ///
    /// ## 重要特性
    /// 1. **条件判断优先**：在每次循环开始前判断条件
    ///    - 条件为 false 时立即退出
    ///    - 可能一次都不执行
    ///
    /// 2. **必须显式可变**：如果循环中要修改变量，需要 `mut`
    ///
    /// ## vs loop
    /// - `loop`：无限循环，需要手动 break
    /// - `while`：条件循环，条件为 false 时自动退出
    ///
    /// ## 典型用途
    /// - 计数器循环
    /// - 等待某个条件满足
    /// - 遍历可选值：`while let Some(x) = opt { }`
    pub fn while_demo() -> i32 {
        let mut num = 0;
        let mut sum = 0;
        while num < 10 {
            num += 1;
            sum += num;
        }
        println!("1+2+...+10 = {}", sum);
        sum
    }
}

// =============================================================================
// 第五章：函数
// =============================================================================

mod chapter05_functions {
    

    /// 知识点 5.1: 函数定义
    pub fn greet(name: &str) -> String {
        format!("你好, {}!", name)
    }

    /// 知识点 5.2: 多参数函数
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// 知识点 5.3: 泛型函数
    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    /// 知识点 5.4: 多返回值（元组）
    pub fn div_mod(a: i32, b: i32) -> (i32, i32) {
        (a / b, a % b)
    }
}

// =============================================================================
// 第六章：数组
// =============================================================================

mod chapter06_array {


    /// 知识点 6.1: 数组定义
    ///
    /// 数组是 Rust 中最简单的固定长度集合，所有元素类型相同。
    ///
    /// ## 语法说明
    /// `let arr: [T; n] = [value1, value2, ...];`
    /// - `T`: 元素类型
    /// - `n`: 数组长度（编译时已知）
    /// - 中括号内是初始值
    ///
    /// ## 重要特性
    /// 1. **固定长度**：长度在编译时确定，不能动态增长
    ///    - 优点：栈上分配，性能高
    ///    - 缺点：长度不灵活
    ///
    /// 2. **类型安全**：编译器会确保不越界访问
    ///    - 运行时仍然可能 panic，但编译器会尽力检查
    ///
    /// 3. **默认初始化**：可以用 `[值; 长度]` 快速初始化
    ///    ```rust
    ///    let zeros = [0; 10];  // 10 个 0
    ///    let same = ["hello"; 3];  // 3 个 "hello"
    ///    ```
    ///
    /// ## 内存布局
    /// - 在栈上连续分配
    /// - 长度 + 数据一次性分配
    /// - 访问效率：O(1)
    pub fn array_definition() -> [i32; 5] {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        println!("数组: {:?}", arr);
        arr
    }

    /// 知识点 6.2: 数组遍历
    ///
    /// 数组的遍历有多种方式，常见的是使用 for 循环。
    ///
    /// ## 遍历方式
    /// 1. **引用遍历**：`for val in &arr`
    ///    - 获取不可变引用，不转移所有权
    ///    - 最常用的方式
    ///
    /// 2. **可变引用**：`for val in &mut arr`
    ///    - 可以修改数组元素
    ///
    /// 3. **所有权遍历**：`for val in arr`
    ///    - 转移所有权，数组之后不能使用
    ///    - 适合不需要继续使用数组的情况
    ///
    /// 4. **enumerate()**：同时获取索引和值
    ///    ```rust
    ///    for (i, val) in arr.iter().enumerate() { }
    ///    ```
    ///
    /// ## iter() 方法
    /// - `iter()`: 返回不可变迭代器
    /// - `iter_mut()`: 返回可变迭代器
    /// - `into_iter()`: 消耗数组，返回值
    pub fn array_iterate() -> i32 {
        let arr = [1, 2, 3, 4, 5];
        let mut sum = 0;
        // 使用引用遍历，避免转移所有权
        for val in &arr {
            sum += val;
        }
        println!("sum = {}", sum);
        sum
    }

    /// 知识点 6.3: 数组切片
    ///
    /// 切片（Slice）是对数组部分或整体的"视图"，不拥有数据。
    ///
    /// ## 语法
    /// - `&arr[start..end]`: 从 start 到 end-1
    /// - `&arr[start..=end]`: 从 start 到 end（含）
    /// - `&arr[..end]`: 从头到 end-1
    /// - `&arr[start..]`: 从 start 到末尾
    /// - `&arr[..]`: 整个数组
    ///
    /// ## 重要特性
    /// 1. **不拥有数据**：切片是引用，指向现有数据
    ///    - 切片本身不分配内存
    ///    - 借用原始数据
    ///
    /// 2. **范围规则**：
    ///    - 索引必须有效（start <= end）
    ///    - 越界会 panic
    ///
    /// 3. **类型**：`&[T]` 表示元素类型为 T 的切片
    ///    - `&[i32]` 是 i32 切片的引用
    ///    - `&str` 是字符串切片
    ///
    /// ## 使用场景
    /// - 函数参数：接收任意长度的数组
    /// - 视图操作：不复制数据的情况下处理部分数据
    /// - 内存高效：不需要复制
    pub fn array_slice() -> i32 {
        let arr = [1, 2, 3, 4, 5];
        // 切片: arr[1], arr[2], arr[3] => 2, 3, 4
        let slice = &arr[1..4];
        println!("slice = {:?}", slice);
        slice.iter().sum()
    }
}

// =============================================================================
// 第七章：Vec 动态数组
// =============================================================================

mod chapter07_vec {


    /// 知识点 7.1: Vec 创建
    ///
    /// Vec（Vector）是 Rust 中最常用的动态数组，可以动态增长和缩小。
    ///
    /// ## 创建方式
    /// 1. **Vec::new()**：显式创建空 Vec，需要类型标注
    ///    ```rust
    ///    let v: Vec<i32> = Vec::new();
    ///    ```
    ///
    /// 2. **vec! 宏**：快速创建并初始化
    ///    ```rust
    ///    let v = vec![1, 2, 3];
    ///    ```
    ///
    /// 3. **vec![值; 长度]**：创建重复值的 Vec
    ///    ```rust
    ///    let v = vec![0; 5];  // [0, 0, 0, 0, 0]
    ///    ```
    ///
    /// ## 内存模型
    /// - **栈上**：Vec 结构体（3 个指针：指针、长度、容量）
    /// - **堆上**：实际数据
    /// - **容量(capacity)**：预分配的内存大小，当数据超过容量时会扩容
    ///
    /// ## 扩容机制
    /// - 容量不足时，Rust 会分配更大的内存（通常是 2 倍）
    /// - 扩容是昂贵的操作，尽量预估值
    /// - 使用 `Vec::with_capacity(capacity)` 预分配
    ///
    /// ## 适用场景
    /// - 长度不确定的集合
    /// - 需要动态添加/删除元素
    /// - 大多数需要"数组"的场景
    pub fn vec_creation() {
        let v1: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
        let v3 = vec![0; 5];
        println!("v1 = {:?}, v2 = {:?}, v3 = {:?}", v1, v2, v3);
    }

    /// 知识点 7.2: Vec 访问
    ///
    /// 访问 Vec 元素有两种方式：索引访问和安全访问。
    ///
    /// ## 访问方式
    /// 1. **索引语法**：`v[index]`
    ///    - 直接访问，越界会 panic
    ///    - 适合确信索引有效的情况
    ///
    /// 2. **get 方法**：`v.get(index)`
    ///    - 返回 `Option<&T>`
    ///    - Some(value) 表示存在，None 表示越界
    ///    - 安全的访问方式
    ///
    /// ## 区别对比
    /// ```rust
    /// let v = vec![10, 20, 30];
    ///
    /// // 索引访问 - 越界会 panic
    /// let first = v[0];  // 10
    /// // let hundred = v[100];  // panic!
    ///
    /// // get 方法 - 越界返回 None
    /// let first = v.get(0);  // Some(&10)
    /// let hundred = v.get(100);  // None
    /// ```
    ///
    /// ## 最佳实践
    /// - 使用 `get` 处理可能越界的情况
    /// - 使用 `match` 或 `if let` 处理 Option
    /// - 使用 `.copied()` 复制值，使用 `.cloned()` 克隆
    pub fn vec_access() -> (Option<i32>, Option<i32>) {
        let v = vec![10, 20, 30];
        // get 返回 Option<&T>，使用 .copied() 复制值
        let first = v.get(0).copied();
        let tenth = v.get(10).copied();
        println!("first = {:?}, tenth = {:?}", first, tenth);
        (first, tenth)
    }

    /// 知识点 7.3: Vec 修改
    ///
    /// Vec 提供了丰富的修改方法，可以动态添加、删除、插入元素。
    ///
    /// ## 常用方法
    /// 1. **push(element)**：在末尾添加元素
    ///    - O(1) 平均，O(n) 最坏（扩容时）
    ///
    /// 2. **pop()**：移除并返回最后一个元素
    ///    - 返回 Option<T>
    ///    - O(1)
    ///
    /// 3. **insert(index, element)**：在指定位置插入
    ///    - O(n)，需要移动后续元素
    ///    - index 必须在 0..=len 范围内
    ///
    /// 4. **remove(index)**：移除并返回指定位置的元素
    ///    - O(n)，需要移动后续元素
    ///
    /// 5. **clear()**：清空所有元素
    ///    - O(n)，会调用元素的 drop
    ///
    /// ## 注意事项
    /// - 只能在可变引用的情况下修改 Vec：`let mut v = vec![...];`
    /// - 频繁在中间插入/删除效率低，考虑使用 LinkedList
    /// - 大量 push 时，预分配容量可以提高性能
    pub fn vec_modify() {
        let mut v = vec![1, 2, 3];
        v.push(4);       // 末尾添加: [1,2,3,4]
        v.pop();         // 移除末尾: 返回 Some(4), v=[1,2,3]
        v.insert(0, 0);  // 开头插入: [0,1,2,3]
        println!("修改后 v = {:?}", v);
    }
}

// =============================================================================
// 第八章：字符串
// =============================================================================

mod chapter08_string {


    /// 知识点 8.1: &str 和 String
    ///
    /// Rust 中有两种字符串类型，理解它们的区别至关重要。
    ///
    /// ## &str（字符串切片）
    /// - **类型**：借用别人的字符串数据
    /// - **本质**：指向字符串数据的指针 + 长度
    /// - **内存**：通常存储在二进制的只读段
    /// - **创建**：`"hello"` 字面量就是 `&str`
    ///
    /// ## String（拥有所有权的字符串）
    /// - **类型**：拥有自己的字符串数据
    /// - **本质**：在堆上分配的字符串，可变
    /// - **内存**：可以动态增长和缩小
    /// - **创建**：`String::from(...)` 或 `.to_string()`
    ///
    /// ## 对比
    /// | 特性 | &str | String |
    /// |------|------|--------|
    /// | 所有权 | 不拥有 | 拥有 |
    /// | 内存位置 | 堆/栈/只读段 | 堆 |
    /// | 可修改 | 否 | 是 |
    /// | 大小 | 16 字节(ptr+len) | 24 字节(ptr+len+cap) |
    /// | 传递成本 | 复制指针(16字节) | 移动或复制(24字节) |
    ///
    /// ## 使用建议
    /// - **函数参数**：优先使用 `&str`，更灵活
    /// - **需要修改**：使用 String
    /// - **存储/返回**：使用 String
    /// - **字面量**：直接用 "hello"（类型是 &str）
    pub fn str_vs_string() {
        let s1: &str = "hello";  // &str: 字符串字面量，borrowed
        let s2: String = String::from("hello");  // String: 堆上分配
        let s3 = "world".to_string();  // String: 从 &str 转换
        println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
    }

    /// 知识点 8.2: 字符串修改
    ///
    /// String 提供了多种修改字符串的方法。
    ///
    /// ## 常用方法
    /// 1. **push(c: char)**：添加单个字符
    /// 2. **push_str(s: &str)**：添加字符串切片
    /// 3. **concat**：`format!` 宏
    /// 4. **replace**：替换子字符串
    ///
    /// ## 扩容
    /// - 当容量不足时，String 会自动扩容
    /// - 通常翻倍扩容
    /// - 可以用 `with_capacity()` 预分配
    ///
    /// ## &str 可以转换 String
    /// ```rust
    /// let s: &str = "hello";
    /// let owned: String = s.to_string();
    /// let owned2: String = String::from(s);
    /// ```
    ///
    /// ## 注意事项
    /// - 只能对 String 使用这些修改方法
    /// - &str 是不可变的
    pub fn string_modify() {
        let mut s = String::from("hello");  // 必须用 mut
        s.push_str(", world!");  // 追加字符串
        println!("s = {}", s);
    }

    /// 知识点 8.3: 字符串方法
    ///
    /// String 提供了丰富的内置方法。
    ///
    /// ## 常用方法
    /// 1. **len()**：返回字节长度，不是字符数！
    ///    - "你好".len() = 6（UTF-8 中每个中文字 3 字节）
    ///    - "hi".len() = 2
    ///
    /// 2. **chars()**：返回字符迭代器
    ///    - "你好".chars().count() = 2
    ///
    /// 3. **trim()**：去除两端空白
    ///    - 空白包括：空格、\t、\n、\r
    ///
    /// 4. **contains(sub)**：是否包含子串
    /// 5. **starts_with/ends_with**：前缀/后缀判断
    /// 6. **replace(old, new)**：替换
    /// 7. **split_whitespace()**：按空白分割
    /// 8. **to_lowercase/to_uppercase()**：大小写转换
    ///
    /// ## 重要提醒
    /// - 处理 Unicode 时，用 chars() 而不是 len()
    /// - len() 是 O(1)，chars().count() 是 O(n)
    pub fn string_methods() {
        let s = "  hello world  ";
        println!("len = {}, trimmed = '{}'", s.len(), s.trim());
    }
}

// =============================================================================
// 第九章：HashMap
// =============================================================================

mod chapter09_hashmap {
    use super::*;

    /// 知识点 9.1: HashMap 创建
    pub fn hashmap_creation() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Alice"), 100);
        scores.insert(String::from("Bob"), 85);
        println!("scores = {:?}", scores);
    }

    /// 知识点 9.2: HashMap 访问
    pub fn hashmap_access() -> Option<i32> {
        let mut scores = HashMap::new();
        scores.insert(String::from("Alice"), 100);
        scores.get("Alice").copied()
    }
}

// =============================================================================
// 第十章：结构体
// =============================================================================

mod chapter10_struct {


    /// 知识点 10.1: 结构体基础
    ///
    /// 结构体（Struct）是将多个相关数据组合在一起的自定义数据类型。
    /// 类似于其他语言的 class，但默认不可变，没有继承。
    ///
    /// ## 定义结构体
    /// ```rust
    /// struct User {
    ///     name: String,   // 字段
    ///     age: u32,
    /// }
    /// ```
    ///
    /// ## 字段
    /// - 每个字段有名称和类型
    /// - 字段默认私有，需要通过 impl 方法访问
    ///
    /// ## 创建实例
    /// ```rust
    /// let user = User {
    ///     name: String::from("Alice"),
    ///     age: 25,
    /// };
    /// ```
    ///
    /// ## 方法（impl 块）
    /// - 使用 `impl` 块为结构体定义方法
    /// - `&self` 表示借用结构体实例
    /// - `&mut self` 表示可变借用
    /// - `self` 表示获取所有权（不常用）
    ///
    /// ## 所有权
    /// - 结构体拥有其字段的所有权
    /// - 如果想引用外部数据，使用生命周期标注
    ///
    /// ## derive 宏
    /// ```rust
    /// #[derive(Debug, Clone, Copy, PartialEq)]
    /// ```
    /// - Debug: 支持 {:?} 打印
    /// - Clone: 支持 .clone()
    /// - Copy: 按位复制（需要所有字段实现 Copy）
    /// - PartialEq: 支持 == 比较

    #[derive(Debug, Clone)]
    pub struct User {
        name: String,  // 私有字段，需要通过方法访问
        #[allow(dead_code)]
        age: u32,
    }

    impl User {
        /// 构造函数：创建新的 User 实例
        /// Self 是 User 的别名，&str 需要转换为 String
        pub fn new(name: &str, age: u32) -> Self {
            User {
                name: name.to_string(),
                age,
            }
        }

        /// 方法：访问结构体数据
        /// &self 表示不可变借用，不获取所有权
        pub fn greet(&self) -> String {
            format!("你好, {}!", self.name)
        }
    }

    /// 元组结构体（Tuple Struct）
    ///
    /// 类似于元组，但有名字，可以给字段命名访问。
    /// 适合简单包装几个值，或者定义新类型。
    ///
    /// ## 特点
    /// - 有名字的元组
    /// - 通过索引访问：color.0, color.1
    /// - 可以实现 trait
    ///
    /// ## 典型用途
    /// - 包装已有类型创造新类型：
    ///   ```rust
    ///   struct Score(i32);  // 和 i32 不同类型
    ///   ```
    /// - 坐标、颜色等简单数据结构
    struct Color(u8, u8, u8);  // RGB 颜色

    pub fn struct_demo() {
        // 创建结构体实例
        let user = User::new("Alice", 25);
        println!("user = {:?}", user);
        println!("greet = {}", user.greet());

        // 创建元组结构体实例
        let color = Color(255, 0, 0);
        println!("color = RGB({}, {}, {})", color.0, color.1, color.2);
    }
}

// =============================================================================
// 第十一章：枚举
// =============================================================================

mod chapter11_enum {


    /// 知识点 11.1: 枚举基础
    ///
    /// 枚举（Enum）定义一个类型可能存在的值集合。
    /// Rust 的枚举非常强大，可以携带数据。
    ///
    /// ## 定义枚举
    /// ```rust
    /// enum Direction {
    ///     Up,      // 单元变体（unit variant）
    ///     Down,
    ///     Left,
    ///     Right,
    /// }
    /// ```
    ///
    /// ## 变体类型
    /// 1. **单元变体**：不带数据，如 `Up`, `Quit`
    /// 2. **元组变体**：带元组数据，如 `Move(x, y)`
    /// 3. **结构体变体**：带结构体数据，如 `Move { x, y }`
    ///
    /// ## 内存表示
    /// - 枚举在 Rust 中是标签联合（tagged union）
    /// - 占用最大变体的空间 + 标签空间
    /// - 编译器确保只有一个变体生效
    ///
    /// ## 方法
    /// 枚举也可以有方法，和结构体一样使用 impl
    ///
    /// ## 强大特性
    /// 枚举可以表示"类型安全"的有限状态：
    /// ```rust
    /// enum Result<T, E> {
    ///     Ok(T),
    ///     Err(E),
    /// }
    /// enum Option<T> {
    ///     Some(T),
    ///     None,
    /// }
    /// ```
    /// 这是 Rust 处理可空值和错误的惯用方式！

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Direction {
        Up, Down, Left, Right,  // 四个方向的变体
    }

    impl Direction {
        /// 为枚举实现方法
        /// 使用 match 匹配当前变体
        pub fn as_str(&self) -> &str {
            match self {
                Direction::Up => "上",
                Direction::Down => "下",
                Direction::Left => "左",
                Direction::Right => "右",
            }
        }
    }

    /// 带数据的枚举
    ///
    /// 枚举的变体可以携带任意数据，这使它非常灵活。
    ///
    /// ## 示例解析
    /// ```rust
    /// enum Message {
    ///     Quit,                      // 单元变体：不需要数据
    ///     Move { x: i32, y: i32 },   // 结构体变体：命名字段
    ///     Write(String),            // 元组变体：匿名字段
    /// }
    /// ```
    ///
    /// ## 使用场景
    /// - **消息类型**：UI 消息、网络消息
    /// - **状态机**：不同状态携带不同数据
    /// - **AST**：抽象语法树
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Message {
        Quit,                              // 不需要数据
        Move { x: i32, y: i32 },           // 命名结构体
        Write(String),                     // 元组
    }

    pub fn enum_demo() {
        let dir = Direction::Up;
        println!("方向 = {}", dir.as_str());

        let msg = Message::Move { x: 10, y: 20 };
        println!("消息 = {:?}", msg);
    }
}

// =============================================================================
// 第十二章：Option 和 Result
// =============================================================================

mod chapter12_option_result {


    /// 知识点 12.1: Option 枚举
    ///
    /// Option 是 Rust 中表示"可能存在或不存在"值的标准库枚举。
    /// 这是一个重要的概念，用于替代其他语言中的 null/nil。
    ///
    /// ## 定义
    /// ```rust
    /// enum Option<T> {
    ///     None,      // 没有值
    ///     Some(T),   // 有值，值是 T
    /// }
    /// ```
    /// - `T` 是泛型，表示 Some 中值的类型
    ///
    /// ## 为什么需要 Option？
    /// - **编译时安全**：编译器强制处理 None 情况
    /// - **明确意图**：`Option<i32>` 明确表示可能是数字也可能是空
    /// - **避免空指针**：再也不会遇到 "cannot read property of null"
    ///
    /// ## 使用方法
    /// 1. **match 模式**：
    ///    ```rust
    ///    match option {
    ///        Some(value) => println!("{}", value),
    ///        None => println!("没有值"),
    ///    }
    ///    ```
    ///
    /// 2. **if let**：简化单分支
    ///    ```rust
    ///    if let Some(value) = option {
    ///        println!("{}", value);
    ///    }
    ///    ```
    ///
    /// 3. **unwrap 系列**：
    ///    - `.unwrap()`: 有值返回，无值 panic
    ///    - `.unwrap_or(default)`: 有值返回，无值用默认值
    ///    - `.unwrap_or_else(f)`: 有值返回，无值调用闭包
    ///
    /// 4. **? 运算符**：传播 None
    ///
    /// ## 常见错误
    /// - 直接使用 Option 而不检查：需要先解包
    /// - 忘记处理 None：编译器会警告
    pub fn option_demo() -> Option<i32> {
        let opt: Option<i32> = Some(5);  // 有值
        let opt2: Option<i32> = None;    // 无值

        if let Some(value) = opt {
            println!("opt 有值: {}", value);
        }

        // .or() 返回 None 时的默认值
        opt2.or(Some(0))
    }

    /// 知识点 12.2: Result 枚举
    ///
    /// Result 是 Rust 中表示"成功或失败"的标准库枚举。
    /// 用于错误处理，是 Rust 的惯用错误处理方式。
    ///
    /// ## 定义
    /// ```rust
    /// enum Result<T, E> {
    ///     Ok(T),   // 成功，值是 T
    ///     Err(E),  // 失败，错误是 E
    /// }
    /// ```
    /// - `T`: 成功时返回的类型
    /// - `E`: 错误类型
    ///
    /// ## 为什么需要 Result？
    /// - **显式错误处理**：调用者必须处理可能的错误
    /// - **类型安全**：错误类型是类型系统的一部分
    /// - **可组合**：可以用 ? 传播错误
    ///
    /// ## 使用方法
    /// 1. **match 模式**
    /// 2. **if let**：简化错误处理
    /// 3. **unwrap 系列**
    /// 4. **? 运算符**（推荐）：传播错误
    ///    ```rust
    ///    fn read_file() -> Result<String, io::Error> {
    ///        let content = fs::read_to_string("file.txt")?;
    ///         // ? 会自动 return Err 如果出错
    ///        Ok(content)
    ///    }
    ///    ```
    ///
    /// ## vs Option
    /// | 场景 | 使用 |
    /// |------|------|
    /// | 值可能不存在 | Option |
    /// | 操作可能失败 | Result |
    /// | 失败原因不重要 | Option |
    /// | 需要错误信息 | Result |
    pub fn result_demo() -> Result<i32, String> {
        let ok: Result<i32, String> = Ok(42);  // 成功
        let _err: Result<i32, String> = Err(String::from("error"));  // 失败

        if let Ok(value) = &ok {
            println!("ok: {}", value);
        }

        ok
    }
}

// =============================================================================
// 第十三章：Trait（特征）
// =============================================================================

mod chapter13_trait {


    /// 知识点 13.1: Trait 基础
    ///
    /// Trait（特征）定义了类型可以实现的抽象行为。
    /// 类似于其他语言的接口（interface）。
    ///
    /// ## 定义 Trait
    /// ```rust
    /// trait Describable {
    ///     fn describe(&self) -> String;  // 方法签名
    /// }
    /// ```
    /// - Trait 定义方法签名
    /// - 实现 Trait 的类型必须提供具体实现
    ///
    /// ## 实现 Trait
    /// ```rust
    /// impl Describable for Point {
    ///     fn describe(&self) -> String {
    ///         format!("点 ({}, {})", self.x, self.y)
    ///     }
    /// }
    /// ```
    /// - 使用 `impl Trait for Type` 语法
    /// - 必须实现所有方法
    ///
    /// ## Trait 的作用
    /// 1. **定义共享行为**：多个类型可以有相同的抽象行为
    /// 2. **泛型约束**：限制泛型必须实现某些 Trait
    /// 3. **默认实现**：可以为方法提供默认实现
    ///
    /// ## 默认方法
    /// ```rust
    /// trait Describable {
    ///     fn describe(&self) -> String {
    ///         String::from("Something")  // 默认实现
    ///     }
    /// }
    /// ```
    ///
    /// ## 常用 Trait
    /// - `Debug`: {:?} 打印
    /// - `Display`: {} 打印
    /// - `Clone`: .clone() 复制
    /// - `Copy`: 按位复制
    /// - `Default`: 默认值
    /// - `PartialEq`: == 比较
    /// - `PartialOrd`: 大小比较

    pub trait Describable {
        fn describe(&self) -> String;  // 方法签名
    }

    #[derive(Debug)]
    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        /// 关联函数：创建新的 Point
        /// Self 是类型的别名
        pub fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
    }

    /// 为 Point 实现 Describable trait
    impl Describable for Point {
        fn describe(&self) -> String {
            format!("点 ({}, {})", self.x, self.y)
        }
    }

    pub fn trait_demo() {
        let p = Point::new(3.0, 4.0);
        println!("p.describe() = {}", p.describe());
    }
}

// =============================================================================
// 第十四章：泛型
// =============================================================================

mod chapter14_generic {


    /// 知识点 14.1: 泛型基础
    ///
    /// 泛型（Generic）允许编写可以适用于多种类型的代码，提高代码复用性。
    /// 泛型在编译时会进行单态化（monomorphization），性能零开销。
    ///
    /// ## 泛型结构体
    /// ```rust
    /// struct Pair<T, U> {
    ///     first: T,
    ///     second: U,
    /// }
    /// ```
    /// - `<T, U>` 定义类型参数
    /// - 可以有多个类型参数，用逗号分隔
    ///
    /// ## 泛型方法
    /// ```rust
    /// impl<T, U> Pair<T, U> {
    ///     fn new(first: T, second: U) -> Self { ... }
    /// }
    /// ```
    /// - impl 后也要声明类型参数
    ///
    /// ## 编译时单态化
    /// Rust 的泛型在编译时会为每种实际类型生成具体代码：
    /// ```rust
    /// let p1 = Pair::new(1, "hello");  // 生成 Pair<i32, &str>
    /// let p2 = Pair::new(3.14, true);  // 生成 Pair<f64, bool>
    /// ```
    /// - **零运行时成本**：和手写具体类型一样快
    /// - **代码膨胀**：每种类型都会生成一份代码
    ///
    /// ## 泛型函数
    /// ```rust
    /// fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
    /// ```
    /// - `<T: PartialOrd>` 是 trait 约束，表示 T 必须可比较

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Pair<T, U> {
        first: T,
        second: U,
    }

    impl<T, U> Pair<T, U> {
        /// 泛型关联函数
        /// 这里的 T 和 U 由调用时的参数类型推断
        pub fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }
    }

    pub fn generic_demo() {
        // 编译器自动推断类型：Pair<i32, &str>
        let p1 = Pair::new(1, "hello");
        // 编译器自动推断类型：Pair<f64, bool>
        let p2 = Pair::new(3.14, true);
        println!("p1 = {:?}", p1);
        println!("p2 = {:?}", p2);
    }
}

// =============================================================================
// 第十五章：生命周期
// =============================================================================

mod chapter15_lifetime {


    /// 知识点 15.1: 生命周期标注
    ///
    /// 生命周期（Lifetime）是 Rust 用来确保引用有效性的机制。
    /// 它告诉编译器引用的有效范围，防止悬垂引用（dangling reference）。
    ///
    /// ## 为什么需要生命周期？
    /// 考虑这个错误代码：
    /// ```rust
    /// fn longest(s1: &str, s2: &str) -> &str {
    ///     if s1.len() > s2.len() { s1 } else { s2 }
    /// }
    /// ```
    /// 编译器不知道返回的引用指向 s1 还是 s2！
    /// 如果返回的是 s1，但 s2 存活更久，函数结束后 s1 可能已失效。
    ///
    /// ## 生命周期标注语法
    /// ```rust
    /// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
    /// ```
    /// - `'a`（读作"生命周期 a"）：声明一个生命周期参数
    /// - `&'a str`：引用在这个生命周期内有效
    /// - 返回值的 `'a` 表示返回值至少和输入中较短的生命周期一样长
    ///
    /// ## 生命周期的规则
    /// 1. 每个引用参数都有自己的生命周期
    /// 2. 如果只有一个输入生命周期，它被赋给所有输出生命周期
    /// 3. 如果有 `&self` 或 `&mut self`，它的生命周期赋给输出
    ///
    /// ## 实际意义
    /// - `'a` 不是具体的时长，而是"关系的描述"
    /// - `'a: 'b` 表示 'a 至少和 'b 一样长
    /// - 编译器使用这些信息验证引用的有效性
    ///
    /// ## 何时需要标注？
    /// - 函数返回引用时
    /// - 结构体包含引用时
    /// - 多个引用之间有关系时
    pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        // 返回值生命周期 = min(s1的生命周期, s2的生命周期)
        if s1.len() > s2.len() { s1 } else { s2 }
    }

    pub fn lifetime_demo() {
        let s1 = "hello";
        let s2 = "world!";
        let result = longest(s1, s2);
        println!("longer = {}", result);
    }
}

// =============================================================================
// 第十六章：闭包
// =============================================================================

mod chapter16_closure {


    /// 知识点 16.1: 闭包基础
    ///
    /// 闭包（Closure）是可以捕获周围环境变量的匿名函数。
    /// 类似于其他语言的 lambda 或匿名函数。
    ///
    /// ## 闭包语法
    /// ```rust
    /// let add = |a, b| a + b;           // 类型推断
    /// let add = |a: i32, b: i32| -> i32 { a + b };  // 显式类型
    /// ```
    /// - `||` 之间是参数
    /// - `->` 之后是返回类型（可省略）
    /// - `{}` 之间是函数体（单表达式可省略）
    ///
    /// ## 捕获环境变量
    /// 闭包可以捕获三种方式：
    /// 1. **不可变借用**：默认，如 `|| println!("{}", x)`
    /// 2. **可变借用**：需要 `|| { x += 1; }`
    /// 3. **获取所有权**：使用 `move` 关键字
    ///
    /// ## Fn, FnMut, FnOnce
    /// Rust 的闭包实现了以下 trait 之一：
    /// - **Fn**：可以多次调用，只读取环境
    /// - **FnMut**：可变调用，可能修改环境
    /// - **FnOnce**：只能调用一次，消耗捕获的值
    ///
    /// ## 与函数的区别
    /// | 特性 | 函数 | 闭包 |
    /// |------|------|------|
    /// | 语法 | fn 关键字 | |x| x+1 |
    /// | 捕获环境 | 否 | 是 |
    /// | 作为参数 | 可直接传递 | 需要 trait bound |
    /// | 性能 | 内联优化 | 内联优化 |
    ///
    /// ## move 关键字
    /// ```rust
    /// let x = vec![1, 2, 3];
    /// let print = move || println!("{:?}", x);
    /// // x 在这里不能再用了！
    /// print();
    /// ```
    /// 用于闭包需要获取所有权时（如在线程中传递）
    pub fn closure_demo() {
        // 闭包作为函数：完整类型标注
        let add = |a: i32, b: i32| -> i32 { a + b };
        println!("add(2, 3) = {}", add(2, 3));

        // 闭包捕获环境变量：不可变借用
        let x = 10;
        let print_x = || println!("x = {}", x);  // 借用 x
        print_x();

        // 闭包捕获环境变量：可变借用
        let mut y = 5;
        let mut inc = || { y += 1; y };  // 可变借用 y
        println!("inc() = {}", inc());
        println!("inc() = {}", inc());
    }
}

// =============================================================================
// 第十七章：迭代器
// =============================================================================

mod chapter17_iterator {


    /// 知识点 17.1: 迭代器基础
    ///
    /// 迭代器（Iterator）是 Rust 中遍历集合的标准方式。
    /// 它是一种惰性的数据流，不会在创建时立即遍历。
    ///
    /// ## Iterator Trait
    /// ```rust
    /// trait Iterator {
    ///     type Item;  // 迭代器的元素类型
    ///     fn next(&mut self) -> Option<Self::Item>;
    /// }
    /// ```
    /// - `next()` 是核心方法，返回 `Option<Item>`
    /// - 返回 `Some(item)` 表示有下一个元素
    /// - 返回 `None` 表示迭代结束
    ///
    /// ## 创建迭代器
    /// - `.iter()`: 产生不可变引用的迭代器
    /// - `.iter_mut()`: 产生可变引用的迭代器
    /// - `.into_iter()`: 消耗集合，产生所有权的迭代器
    ///
    /// ## 常用适配器（Adapter）
    /// 这些方法返回新迭代器，不执行遍历（惰性）：
    /// - `.map(f)`: 转换每个元素
    /// - `.filter(p)`: 过滤元素
    /// - `.take(n)`: 取前 n 个
    /// - `.skip(n)`: 跳过前 n 个
    /// - `.enumerate()`: 添加索引
    /// - `.rev()`: 反向
    /// - `.zip(other)`: 合并两个迭代器
    ///
    /// ## 消费器（Consumer）
    /// 这些方法真正执行遍历：
    /// - `.collect()`: 收集到集合
    /// - `.fold(acc, f)`: 折叠/归约
    /// - `.sum()`, `.product()`: 求和/积
    /// - `.find()`, `.any()`, `.all()`: 查找/断言
    ///
    /// ## 链式调用
    /// ```rust
    /// let result = (1..=10)
    ///     .filter(|x| x % 2 == 0)  // 偶数
    ///     .map(|x| x * x)          // 平方
    ///     .sum::<i32>();           // 求和
    /// ```
    pub fn iterator_demo() {
        let numbers = vec![1, 2, 3, 4, 5];

        // map: 转换每个元素，生成新迭代器
        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        println!("翻倍: {:?}", doubled);

        // sum: 消费迭代器，求和
        let sum: i32 = numbers.iter().sum();
        println!("sum = {}", sum);

        // filter: 过滤元素，cloned() 复制值避免借用
        let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
        println!("偶数: {:?}", evens);
    }

    /// 知识点 17.2: 自定义迭代器
    ///
    /// 任何类型只要实现了 Iterator trait，就可以使用 for 循环。
    /// 这是 Rust 的强大特性，允许为自定义类型创建迭代器。
    ///
    /// ## 实现 Iterator
    /// ```rust
    /// impl Iterator for Counter {
    ///     type Item = u32;  // 元素类型
    ///
    ///     fn next(&mut self) -> Option<Self::Item> {
    ///         // 返回下一个元素或 None
    ///     }
    /// }
    /// ```
    ///
    /// ## 迭代器状态
    /// - 迭代器通常保存状态来跟踪当前位置
    /// - 每次调用 next() 更新状态
    /// - 状态可以是内部字段或可变引用
    ///
    /// ## 使用迭代器适配器
    /// 自定义迭代器可以自动使用所有 Iterator 方法：
    /// ```rust
    /// let sum = counter.filter(|x| x > 2).map(|x| x * 2).sum();
    /// ```
    ///
    /// ## 优势
    /// - 惰性求值：只在需要时计算
    /// - 零成本抽象：和手写循环性能相同
    /// - 可组合：丰富的方法链
    struct Counter {
        current: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Self {
            Counter { current: 0, max }
        }
    }

    /// 为 Counter 实现 Iterator trait
    impl Iterator for Counter {
        type Item = u32;  // 迭代器产生的元素类型

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                self.current += 1;
                Some(self.current)
            } else {
                None
            }
        }
    }

    pub fn custom_iterator() {
        let counter = Counter::new(5);
        // 可以直接使用 sum() 等方法
        let sum: u32 = counter.sum();
        println!("Counter sum = {}", sum);
    }
}

// =============================================================================
// 第十八章：智能指针
// =============================================================================

mod chapter18_smart_pointer {

    use std::cell::RefCell;
    use std::rc::Rc;

    /// 知识点 18.1: Box<T> - 堆上分配的智能指针
    ///
    /// Box 是最简单的智能指针，将数据存储在堆上而不是栈上。
    /// 相当于 C 的 malloc/free 或其他语言的 boxing。
    ///
    /// ## 使用场景
    /// 1. **递归类型**：类型大小编译时未知
    ///    ```rust
    ///    enum List {
    ///        Cons(i32, Box<List>),  // 需要 Box，否则大小无限
    ///        Nil,
    ///    }
    ///    ```
    ///
    /// 2. **大数据转移**：避免栈拷贝
    ///    - 大结构体在栈上拷贝昂贵
    ///    - Box 只拷贝指针
    ///
    /// 3. ** trait 对象**：动态分发
    ///    ```rust
    ///    trait Animal { ... }
    ///    struct Dog;
    ///    let animal: Box<dyn Animal> = Box::new(Dog);
    ///    ```
    ///
    /// ## 特点
    /// - 零大小开销：只是指针的封装
    /// - 堆分配：有额外的堆分配开销
    /// - 确定性析构：离开作用域自动释放
    ///
    /// ## 解引用
    /// - `*box` 获取值
    /// - 自动解引用：方法调用会解引用
    pub fn box_demo() -> i32 {
        let b = Box::new(5);  // 堆上分配 i32
        println!("b = {}", b);
        *b  // 解引用获取值
    }

    /// 知识点 18.2: Rc<T> - 引用计数智能指针
    ///
    /// Rc（Reference Counted）允许多个所有者共享数据。
    /// 引用计数跟踪数据有多少个所有者，归零时释放数据。
    ///
    /// ## 特点
    /// - **单线程**：只适合单线程，多线程用 Arc
    /// - **不可变共享**：数据不可修改（需要 RefCell）
    /// - **引用计数**：增加/减少引用计数有开销
    ///
    /// ## 使用场景
    /// - 多个地方需要同一份数据
    /// - 数据的所有权不明确
    /// - 避免循环引用
    ///
    /// ## 方法
    /// - `Rc::new(data)`：创建新的 Rc
    /// - `Rc::clone(&rc)`：克隆（增加引用计数）
    /// - `Rc::strong_count(&rc)`：获取引用计数
    /// - `Rc::downgrade(&rc)`：创建 Weak 引用
    ///
    /// ## 注意
    /// - 循环引用会导致内存泄漏（使用 Weak 解决）
    /// - 性能开销：每次克隆有原子操作
    pub fn rc_demo() -> usize {
        let data = Rc::new(vec![1, 2, 3]);  // 引用计数 = 1
        let _clone1 = Rc::clone(&data);      // 引用计数 = 2
        let count = Rc::strong_count(&data); // 获取当前计数
        println!("引用计数: {}", count);
        count
    }

    /// 知识点 18.3: RefCell<T> - 内部可变性
    ///
    /// RefCell 提供了"内部可变性"（Interior Mutability）。
    /// 即使数据不可变引用，也能通过 RefCell 修改数据。
    ///
    /// ## 问题场景
    /// ```rust
    /// let x = 5;
    /// let rx = &x;
    /// // 无法修改：*rx = 10; // 错误！
    /// ```
    ///
    /// ## RefCell 解决方案
    /// ```rust
    /// let x = RefCell::new(5);
    /// *x.borrow_mut() = 10;  // 可以修改！
    /// ```
    ///
    /// ## 运行时分发
    /// - 借用检查在编译时：常规引用
    /// - 借用检查在运行时：RefCell
    /// - 违反了规则会 panic，而不是编译错误
    ///
    /// ## 使用场景
    /// - 模拟可变状态（如 Rc<RefCell<T>>）
    /// - 缓存等需要内部修改的场景
    /// - 适配现有 API
    ///
    /// ## 方法
    /// - `borrow()`：获取不可变引用，返回 Ref<T>
    /// - `borrow_mut()`：获取可变引用，返回 RefMut<T>
    /// - 引用离开作用域自动归还
    pub fn refcell_demo() -> i32 {
        let x = RefCell::new(5);
        // borrow_mut() 获取可变引用
        *x.borrow_mut() += 10;
        // borrow() 获取不可变引用
        let result = *x.borrow();
        result
    }
}

// =============================================================================
// 第十九章：错误处理
// =============================================================================

mod chapter19_error {


    /// 知识点 19.1: 错误处理
    ///
    /// Rust 的错误处理分为两类：可恢复错误和不可恢复错误。
    ///
    /// ## 1. 可恢复错误（Result）
    /// 用于可能失败但需要处理的操作：
    /// - 文件操作：可能不存在权限
    /// - 网络请求：可能超时
    /// - 解析输入：可能格式错误
    ///
    /// ## 2. 不可恢复错误（panic!）
    /// 用于程序无法继续的情况：
    /// - 索引越界
    /// - 断言失败
    /// - 逻辑错误
    ///
    /// ## ? 运算符
    /// - 传播错误的便捷方式
    /// - 相当于 `match` 但更简洁
    /// ```rust
    /// fn read_file() -> Result<String, io::Error> {
    ///     let content = fs::read_to_string("file.txt")?;
    ///     // 如果 Err，自动返回
    ///     Ok(content)
    /// }
    /// ```
    /// - 只能在返回 Result 的函数中使用
    ///
    /// ## 最佳实践
    /// 1. 使用 Result 处理预期错误
    /// 2. 使用 panic! 处理不应该发生的情况
    /// 3. 提供有意义的错误信息
    /// 4. 避免在库中 panic，让调用者决定
    ///
    /// ## 常用方法
    /// - `.unwrap()`: 有值返回，无值 panic
    /// - `.expect(msg)`: 自定义 panic 信息
    /// - `.unwrap_or(default)`: 默认值
    /// - `.unwrap_or_else(f)`: 闭包处理
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        // ? 运算符：成功返回 Ok(num * 2)，失败自动传播错误
        let num: i32 = s.parse()?;
        Ok(num * 2)
    }

    pub fn error_handling() -> Result<i32, std::num::ParseIntError> {
        // 使用 ? 传播错误
        let result = parse_number("42")?;
        println!("result = {}", result);
        Ok(result)
    }
}

// =============================================================================
// 第二十章：函数式编程
// =============================================================================

mod chapter20_functional {
    

    pub fn functional_demo() {
        let numbers = vec![1, 2, 3, 4, 5];

        // map
        let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
        println!("map: {:?}", doubled);

        // filter
        let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
        println!("filter: {:?}", evens);

        // fold
        let sum = numbers.iter().fold(0, |acc, x| acc + x);
        println!("fold sum: {}", sum);

        // 链式调用
        let result = (1..=10)
            .filter(|x| x % 2 == 0)
            .map(|x| x * x)
            .sum::<i32>();
        println!("1-10 偶数的平方和 = {}", result);
    }
}

// =============================================================================
// 第二十一章：unsafe 块
// =============================================================================

mod chapter21_unsafe {


    /// 知识点 21.1: unsafe 块
    ///
    /// unsafe 块允许使用 Rust 中被正常情况禁止的操作。
    /// 这不是"不安全的 Rust"，而是"Rust 中允许不安全操作的区域"。
    ///
    /// ## 为什么需要 unsafe？
    /// Rust 的安全特性很好，但有些操作必须绕过它：
    /// 1. **原始指针**：解引用原始指针
    /// 2. **FFI**：调用其他语言（C/C++）
    /// 3. **底层操作**：直接内存操作
    /// 4. **不安全的 trait**：实现 unsafe trait
    ///
    /// ## unsafe 可以做的事
    /// 1. **解引用原始指针**
    ///    ```rust
    ///    let raw_ptr = vec.as_ptr();
    ///    unsafe { *raw_ptr };
    ///    ```
    ///
    /// 2. **调用 unsafe 函数**
    ///    ```rust
    ///    unsafe { std::arch::asm!("nop") };
    ///    ```
    ///
    /// 3. **实现 unsafe trait**
    ///    ```rust
    ///    unsafe trait SafeSend {}
    ///    ```
    ///
    /// 4. **访问或修改可变静态变量**
    ///    ```rust
    ///    static mut COUNTER: i32 = 0;
    ///    unsafe { COUNTER += 1; }
    ///    ```
    ///
    /// ## 安全责任
    /// - unsafe 不关闭借用检查器
    /// - 你必须保证代码是正确的
    /// - 错误会导致未定义行为（UB）
    /// - 注释清楚说明你的假设
    ///
    /// ## 最佳实践
    /// 1. 最小化 unsafe 代码
    /// 2. 将 unsafe 封装在安全接口中
    /// 3. 文档化所有不变量
    /// 4. 测试边界情况
    pub fn unsafe_demo() {
        let nums = [1, 2, 3];
        // 获取数组的原始指针
        let ptr = nums.as_ptr();

        unsafe {
            // 在 unsafe 块中解引用原始指针
            println!("通过原始指针访问: {}", *ptr);
        }
    }
}

// =============================================================================
// 程序入口
// =============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    Rust 系统性学习教程                         ║");
    println!("║              循序渐进 · 详尽注释 · 完整知识点                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // 展示各章节内容
    println!("【第一章】变量与常量");
    chapter01_variables::shadowing();
    println!();

    println!("【第二章】基本数据类型");
    chapter02_types::integer_types();
    chapter02_types::floating_types();
    chapter02_types::char_type();
    chapter02_types::integer_literals();
    chapter02_types::type_casting();
    println!();

    println!("【第三章】运算符");
    chapter03_operators::arithmetic_operators();
    chapter03_operators::bitwise_operators();
    println!();

    println!("【第四章】控制流");
    chapter04_control_flow::if_else_demo();
    chapter04_control_flow::match_demo();
    chapter04_control_flow::for_demo();
    chapter04_control_flow::loop_demo();
    chapter04_control_flow::while_demo();
    println!();

    println!("【第五章】函数");
    println!("greet: {}", chapter05_functions::greet("小明"));
    println!("add: {}", chapter05_functions::add(3, 4));
    let (q, r) = chapter05_functions::div_mod(10, 3);
    println!("div_mod: 10/3 = {} 余 {}", q, r);
    // 泛型函数
    let nums = vec![3, 1, 4, 1, 5];
    println!("largest: {}", chapter05_functions::largest(&nums));
    println!();

    println!("【第六章】数组");
    chapter06_array::array_definition();
    println!("sum = {}", chapter06_array::array_iterate());
    println!("slice sum = {}", chapter06_array::array_slice());
    println!();

    println!("【第七章】Vec");
    chapter07_vec::vec_creation();
    chapter07_vec::vec_modify();
    chapter07_vec::vec_access();
    println!();

    println!("【第八章】字符串");
    chapter08_string::str_vs_string();
    chapter08_string::string_modify();
    chapter08_string::string_methods();
    println!();

    println!("【第九章】HashMap");
    chapter09_hashmap::hashmap_creation();
    println!("Alice 分数: {:?}", chapter09_hashmap::hashmap_access());
    println!();

    println!("【第十章】结构体");
    chapter10_struct::struct_demo();
    println!();

    println!("【第十一章】枚举");
    chapter11_enum::enum_demo();
    println!();

    println!("【第十二章】Option 与 Result");
    chapter12_option_result::option_demo();
    let _ = chapter12_option_result::result_demo();
    println!();

    println!("【第十三章】Trait");
    chapter13_trait::trait_demo();
    println!();

    println!("【第十四章】泛型");
    chapter14_generic::generic_demo();
    println!();

    println!("【第十五章】生命周期");
    chapter15_lifetime::lifetime_demo();
    println!();

    println!("【第十六章】闭包");
    chapter16_closure::closure_demo();
    println!();

    println!("【第十七章】迭代器");
    chapter17_iterator::iterator_demo();
    chapter17_iterator::custom_iterator();
    println!();

    println!("【第十八章】智能指针");
    println!("box = {}", chapter18_smart_pointer::box_demo());
    println!("rc count = {}", chapter18_smart_pointer::rc_demo());
    println!("refcell = {}", chapter18_smart_pointer::refcell_demo());
    println!();

    println!("【第十九章】错误处理");
    let _ = chapter19_error::error_handling();
    println!();

    println!("【第二十章】函数式编程");
    chapter20_functional::functional_demo();
    println!();

    println!("【第二十一章】unsafe");
    chapter21_unsafe::unsafe_demo();
    println!();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                      祝您学习愉快！                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}