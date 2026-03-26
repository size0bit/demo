// =============================================================================
// 测试模块 - 对 chapters.rs 中所有函数的调用测试
// =============================================================================

mod chapters;

#[test]
fn it_works() {
    chapters::chapter01_variables::shadowing();
}

// =============================================================================
// 第一章：变量与常量
// =============================================================================

#[test]
fn test_chapter01_variables() {
    // 测试 shadowing - 变量遮蔽
    let len = chapters::chapter01_variables::shadowing();
    assert!(len > 0, "shadowing 应返回有效长度");

    // 测试 basic_variable - 基础变量声明
    let x = chapters::chapter01_variables::basic_variable();
    assert_eq!(x, 5, "basic_variable 应返回 5");

    // 测试 mutable_variable - 可变变量
    let y = chapters::chapter01_variables::mutable_variable();
    assert_eq!(y, 6, "mutable_variable 应返回 6");

    // 测试 constant_demo - 常量
    let max = chapters::chapter01_variables::constant_demo();
    assert_eq!(max, 100, "constant_demo 应返回 MAX_SIZE");
}

// =============================================================================
// 第二章：基本数据类型
// =============================================================================

#[test]
fn test_chapter02_types() {
    // 测试 integer_types - 整数类型
    let (a, b, c, d) = chapters::chapter02_types::integer_types();
    assert_eq!(a, 42);
    assert_eq!(b, 100);
    assert_eq!(c, -123);
    assert_eq!(d, 999);

    // 测试 floating_types - 浮点类型
    let (x, y, z) = chapters::chapter02_types::floating_types();
    assert_eq!(x, 2.0);
    assert_eq!(y, 3.0);
    assert_eq!(z, -1.5);

    // 测试 char_type - 字符类型
    let (c1, c2, c3) = chapters::chapter02_types::char_type();
    assert_eq!(c1, 'a');
    assert_eq!(c2, '中');
    assert_eq!(c3, '😀');

    // 测试 integer_literals - 整数字面量
    let (decimal, hex, octal, binary, byte) = chapters::chapter02_types::integer_literals();
    assert_eq!(decimal, 98_222);
    assert_eq!(hex, 0xff);
    assert_eq!(octal, 0o77);
    assert_eq!(binary, 0b1111_0000);
    assert_eq!(byte, b'A');

    // 测试 type_casting - 类型转换
    let (int_val, _, _) = chapters::chapter02_types::type_casting();
    assert_eq!(int_val, 3, "3.99 as i32 应截断为 3");
}

// =============================================================================
// 第三章：运算符
// =============================================================================

#[test]
fn test_chapter03_operators() {
    // 测试 arithmetic_operators - 算术运算符
    let (sum, diff, product, quotient, remainder) = chapters::chapter03_operators::arithmetic_operators();
    assert_eq!(sum, 13);   // 10 + 3
    assert_eq!(diff, 7);   // 10 - 3
    assert_eq!(product, 30); // 10 * 3
    assert_eq!(quotient, 3); // 10 / 3
    assert_eq!(remainder, 1); // 10 % 3

    // 测试 relational_operators - 关系运算符
    let results = chapters::chapter03_operators::relational_operators();
    assert_eq!(results, vec![false, true, true, false, true, false]);

    // 测试 logical_operators - 逻辑运算符
    let (and_result, or_result, not_result) = chapters::chapter03_operators::logical_operators();
    assert_eq!(and_result, false);  // true && false
    assert_eq!(or_result, true);    // true || false
    assert_eq!(not_result, true);   // !false

    // 测试 bitwise_operators - 位运算符
    let (and_val, or_val, xor_val, shl_val, shr_val) = chapters::chapter03_operators::bitwise_operators();
    assert_eq!(and_val, 0b1000);    // 1100 & 1010
    assert_eq!(or_val, 0b1110);     // 1100 | 1010
    assert_eq!(xor_val, 0b0110);    // 1100 ^ 1010
    assert_eq!(shl_val, 0b110000);  // 1100 << 2
    assert_eq!(shr_val, 0b0011);    // 1100 >> 2
}

// =============================================================================
// 第四章：控制流
// =============================================================================

#[test]
fn test_chapter04_control_flow() {
    // 测试 if_else_demo - if-else 条件分支
    chapters::chapter04_control_flow::if_else_demo();

    // 测试 match_demo - match 模式匹配
    let day_name = chapters::chapter04_control_flow::match_demo();
    assert_eq!(day_name, "星期三");

    // 测试 for_demo - for 循环
    let sum = chapters::chapter04_control_flow::for_demo();
    assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5 = 15

    // 测试 loop_demo - loop 循环
    let result = chapters::chapter04_control_flow::loop_demo();
    assert_eq!(result, 10); // count=5, 5*2=10

    // 测试 while_demo - while 循环
    let sum_while = chapters::chapter04_control_flow::while_demo();
    assert_eq!(sum_while, 55); // 1+2+...+10 = 55
}

// =============================================================================
// 第五章：函数
// =============================================================================

#[test]
fn test_chapter05_functions() {
    // 测试 greet - 问候函数
    let greeting = chapters::chapter05_functions::greet("小明");
    assert_eq!(greeting, "你好, 小明!");

    // 测试 add - 加法函数
    let sum = chapters::chapter05_functions::add(3, 4);
    assert_eq!(sum, 7);

    // 测试 div_mod - 除法取余（多返回值）
    let (q, r) = chapters::chapter05_functions::div_mod(10, 3);
    assert_eq!(q, 3);  // 10 / 3 = 3
    assert_eq!(r, 1);  // 10 % 3 = 1

    // 测试 largest - 泛型函数
    let nums = vec![3, 1, 4, 1, 5];
    let largest = chapters::chapter05_functions::largest(&nums);
    assert_eq!(*largest, 5);
}

// =============================================================================
// 第六章：数组
// =============================================================================

#[test]
fn test_chapter06_array() {
    // 测试 array_definition - 数组定义
    let arr = chapters::chapter06_array::array_definition();
    assert_eq!(arr, [1, 2, 3, 4, 5]);

    // 测试 array_iterate - 数组遍历
    let sum = chapters::chapter06_array::array_iterate();
    assert_eq!(sum, 15); // 1+2+3+4+5 = 15

    // 测试 array_slice - 数组切片
    let slice_sum = chapters::chapter06_array::array_slice();
    assert_eq!(slice_sum, 9); // 2+3+4 = 9
}

// =============================================================================
// 第七章：Vec
// =============================================================================

#[test]
fn test_chapter07_vec() {
    // 测试 vec_creation - Vec 创建
    chapters::chapter07_vec::vec_creation();

    // 测试 vec_modify - Vec 修改
    chapters::chapter07_vec::vec_modify();

    // 测试 vec_access - Vec 访问
    let (first, tenth) = chapters::chapter07_vec::vec_access();
    assert_eq!(first, Some(10));
    assert_eq!(tenth, None);
}

// =============================================================================
// 第八章：字符串
// =============================================================================

#[test]
fn test_chapter08_string() {
    // 测试 str_vs_string - &str 和 String 的区别
    chapters::chapter08_string::str_vs_string();

    // 测试 string_modify - 字符串修改
    chapters::chapter08_string::string_modify();

    // 测试 string_methods - 字符串方法
    chapters::chapter08_string::string_methods();
}

// =============================================================================
// 第九章：HashMap
// =============================================================================

#[test]
fn test_chapter09_hashmap() {
    // 测试 hashmap_creation - HashMap 创建
    chapters::chapter09_hashmap::hashmap_creation();

    // 测试 hashmap_access - HashMap 访问
    let alice_score = chapters::chapter09_hashmap::hashmap_access();
    assert_eq!(alice_score, Some(100));
}

// =============================================================================
// 第十章：结构体
// =============================================================================

#[test]
fn test_chapter10_struct() {
    // 测试 struct_demo - 结构体演示
    chapters::chapter10_struct::struct_demo();
}

// =============================================================================
// 第十一章：枚举
// =============================================================================

#[test]
fn test_chapter11_enum() {
    // 测试 enum_demo - 枚举演示
    chapters::chapter11_enum::enum_demo();
}

// =============================================================================
// 第十二章：Option 与 Result
// =============================================================================

#[test]
fn test_chapter12_option_result() {
    // 测试 option_demo - Option 枚举
    let opt_result = chapters::chapter12_option_result::option_demo();
    assert_eq!(opt_result, Some(0)); // None.or(Some(0)) = Some(0)

    // 测试 result_demo - Result 枚举
    let result = chapters::chapter12_option_result::result_demo();
    assert_eq!(result, Ok(42));
}

// =============================================================================
// 第十三章：Trait
// =============================================================================

#[test]
fn test_chapter13_trait() {
    // 测试 trait_demo - Trait 演示
    chapters::chapter13_trait::trait_demo();
}

// =============================================================================
// 第十四章：泛型
// =============================================================================

#[test]
fn test_chapter14_generic() {
    // 测试 generic_demo - 泛型演示
    chapters::chapter14_generic::generic_demo();
}

// =============================================================================
// 第十五章：生命周期
// =============================================================================

#[test]
fn test_chapter15_lifetime() {
    // 测试 lifetime_demo - 生命周期演示
    chapters::chapter15_lifetime::lifetime_demo();
}

// =============================================================================
// 第十六章：闭包
// =============================================================================

#[test]
fn test_chapter16_closure() {
    // 测试 closure_demo - 闭包演示
    chapters::chapter16_closure::closure_demo();
}

// =============================================================================
// 第十七章：迭代器
// =============================================================================

#[test]
fn test_chapter17_iterator() {
    // 测试 iterator_demo - 迭代器基础
    chapters::chapter17_iterator::iterator_demo();

    // 测试 custom_iterator - 自定义迭代器
    chapters::chapter17_iterator::custom_iterator();
}

// =============================================================================
// 第十八章：智能指针
// =============================================================================

#[test]
fn test_chapter18_smart_pointer() {
    // 测试 box_demo - Box 智能指针
    let box_val = chapters::chapter18_smart_pointer::box_demo();
    assert_eq!(box_val, 5);

    // 测试 rc_demo - Rc 引用计数
    let rc_count = chapters::chapter18_smart_pointer::rc_demo();
    assert!(rc_count >= 2, "Rc 引用计数应该至少为 2");

    // 测试 refcell_demo - RefCell 内部可变性
    let refcell_val = chapters::chapter18_smart_pointer::refcell_demo();
    assert_eq!(refcell_val, 15); // 5 + 10 = 15
}

// =============================================================================
// 第十九章：错误处理
// =============================================================================

#[test]
fn test_chapter19_error() {
    // 测试 error_handling - 错误处理
    let result = chapters::chapter19_error::error_handling();
    assert_eq!(result, Ok(84)); // "42".parse::<i32>()? * 2 = 84
}

// =============================================================================
// 第二十章：函数式编程
// =============================================================================

#[test]
fn test_chapter20_functional() {
    // 测试 functional_demo - 函数式编程演示
    chapters::chapter20_functional::functional_demo();
}

// =============================================================================
// 第二十一章：unsafe
// =============================================================================

#[test]
fn test_chapter21_unsafe() {
    // 测试 unsafe_demo - unsafe 块演示
    chapters::chapter21_unsafe::unsafe_demo();
}

fn main() {}