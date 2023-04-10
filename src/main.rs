// #region hello world ---start
// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";

//     let regions = [southern_germany, chinese, english];

//     // for region in regions.iter() {
//     //     print!("{}", &region)
//     // }
//     for region in regions {
//         print!("{}", &region)
//     }
// }
// #endregion hello world ---end

// #region complex ---start
// fn complex() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();

//     for (index, record) in records.enumerate() {
//         if index == 0 || record.trim().len() == 0 {
//             continue;
//         }

//         // 声明一个 fields 变量，类型是 Vec
//         // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//         // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//         let fileds: Vec<_> = record.split(",").map(|field| field.trim()).collect();

//         if cfg!(debug_assertions) {
//             eprint!("debug: {:?} -> {:?}", record, fileds)
//         }

//         let name = fileds[0];

//         // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量

//         // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//         // 1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，
//         // if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//         // 2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length

//         // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//         if let Ok(length) = fileds[1].parse::<f32>() {
//             println!("{},{}cm", name, length)
//         }
//     }
// }
// #endregion complex ---end

// #region base ---start
// fn base() {
//     // 使用let来声明变量，进行绑定，a是不可变的
//     // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
//     // 语句的末尾必须以分号结尾
//     let a = 10;

//     // 主动指定b的类型为i32
//     let b: i32 = 20;

//     // 这里有两点值得注意：
//     // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
//     // 2. c是可变的，mut是mutable的缩写
//     let mut c = 30i32;

//     // 还能在数值和类型中间添加一个下划线，让可读性更好
//     let d = 30_i32;

//     let e = add(add(a, b), add(c, d));

//     println!("( a + b ) + ( c + d ) = {}", e);
//     print!("{}", e);
// }

// // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
// fn add(i: i32, j: i32) -> i32 {
//     // 返回相加值，这里可以省略return
//     // i + j

//     return i + j;
// }
// #endregion base ---end

// #region variables ---start
// fn variables() {
//     let mut x = 5;

//     println!("The value of x is: {}", x);

//     x = 6;

//     println!("The value of x is: {}", x);
// }
// #endregion variables ---end

// #region shadowing ---start
// fn shadowing() {
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;

//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }

//     println!("The value of x is: {}", x)
// }
// #endregion shadowing ---end

// #region range ---start
// fn range() {
//     for value in 'a'..='z' {
//         println!("{}", value)
//     }
// }
// #endregion range ---end

// #region plusOrMinus ---start
fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}
// #endregion plusOrMinus ---end

// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
fn main() {
    // println!("Hello, world!");
    // greet_world();
    // complex();
    // base();
    // variables();
    // shadowing();
    // range();
    println!("The value of x is: {}", plus_or_minus(6));
}
