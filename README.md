# Rust 语法演示项目

## 位置
`/home/z/Documents/demo/src/main.rs`

## 内容
完整的 Rust 核心语法演示，约 500 行，包含：

- **基础语法**: 变量、可变性(shadowing)、常量、基本类型、字符串
- **控制流**: if-else、match（多值匹配、守卫）、for/while/loop
- **集合类型**: 数组、Vec、HashMap
- **结构体**: 定义、方法、关联函数、derive 宏
- **枚举**: 模式匹配、关联数据、methods
- **Trait**: 定义、实现、为外部类型实现、默认方法
- **泛型**: 泛型函数、泛型结构体、trait bounds
- **生命周期**: `'a` 标记、longest 函数、结构体中的生命周期
- **闭包**: 环境捕获、Fn/FnMut/FnOnce、move 关键字
- **迭代器**: 适配器(map/filter)、消费器(sum/fold)、自定义迭代器
- **智能指针**: Box、Rc、RefCell
- **错误处理**: Option、Result、? 运算符
- **其他**: 类型别名、#[must_use]、unsafe、static

## 运行
```bash
cd /home/z/Documents/demo
cargo run
```

---
*保存于 2026-03-26*
