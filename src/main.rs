mod mod1 {
    pub const MESSAGE: &str = "Hello, world!";

    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello, world2!";
    }
}

use std::collections::HashMap;
use std::mem;
use std::thread;
const A: i32 = 10;

fn get_number() -> i32 {
    42
}

fn main() {
    println!("{}", mod1::MESSAGE);
    println!("{}", mod1::mod2::MESSAGE);

    println!("{}", mod1::CrateEnum::Item as u32);

    let mut x = 5;
    println!("{}", x);

    x = 6;
    println!("{}", x);

    println!("{}", A);

    let r = get_number();
    println!("{}", r);

    //这两种写法等价
    let a = 1;
    let a = a + 1;

    let mut b = 1;
    b = b + 1;

    let mytuple: (i32, char) = (109, 'a');
    let (c, d) = mytuple;
    // println!("{} {}", c, d);
    println!("{}", mytuple.0);
    println!("{}", mytuple.1);

    let (result, is_overflow) = mytuple.0.overflowing_add(10);
    println!("{} {}", result, is_overflow);

    // let index = "5".parse::<usize>().unwrap();
    // let array: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", array[index]);

    let mut mybuffer: [i32; 32 * 1024] = [0; 32 * 1024];
    mybuffer[1024] = 13;

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    let slice2 = &arr[3..5];

    slice2.len();
    slice2.is_empty();

    let mut slice3 = &mut arr[..];
    slice3[0] = 6;
    println!("{}", arr[0]);

    struct Pair(i32, f32);
    let pair = Pair(10, 4.2);
    println!("{}", pair.0);

    #[derive(Debug)] //派生属性
    struct Person {
        name: String,
        age: u32,
    }
    let jack = Person {
        name: String::from("Jack"),
        age: 6,
    };
    println!("{} {}", jack.name, jack.age);
    println!("{:?}", jack);

    struct Unit;
    let unit = Unit;

    enum IPAddr {
        IPV4(u8, u8, u8, u8),
        IPV6(
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
        ),
    }

    let localhost: IPAddr = IPAddr::IPV4(127, 0, 0, 1);
    match localhost {
        IPAddr::IPV4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {}
    };

    let a: i8 = -10;
    let b = a as u8;
    println!("{}", b);

    // as 只允许安全的转换
    // transmute，需要将代码写入unsafe中
    unsafe {
        let a1 = [0u8, 1u8, 0u8, 0u8];
        let b1: u32 = mem::transmute(a1);
        println!("{}", b1);
    }

    let n = 5;
    if n > 0 {
        println!("is positive");
    } else if n < 0 {
        println!("is negative");
    } else {
        println!("is zero");
    }

    let m = if n < 0 { 2.0 } else { 3.0 };
    println!("{}", m);

    let mut sum = 0;
    let mut n1 = 0;
    loop {
        sum += n1;
        n1 += 1;
        if n1 > 100 {
            break;
        }
    }
    println!("{}", sum);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 1;
        }
    };
    println!("{}", result);

    //while就是个带条件的loop
    // let mut n3 = 0;
    // while n3 < 2 {
    //     if n3 == 1 {
    //         println!("{}", n3);
    //     }
    //     n3 += 1;
    // }

    for i in 1..6 {
        println!("{}", i);
    }
    println!("--------");
    for i in 1..=6 {
        println!("{}", i);
    }

    let marray = ["a", "b", "c"];
    for i in marray.iter() {
        println!("{}", i);
    }
    let mut marray2 = [1, 2, 3];
    for i in marray2.iter_mut() {
        *i *= 2;
        println!("{}", i);
    }

    let opcode: u8 = 42;
    match opcode {
        42 => {
            println!("bingo!");
        }
        _ => {
            println!("{}", opcode);
        }
    }

    enum Alphabet {
        A,
        B,
        C,
    }
    let letter = Alphabet::A;
    if let Alphabet::A = letter {
        println!("It's A")
    }

    enum Symbol {
        Char(char),
        Number,
    }
    let l2 = Symbol::Char('z');
    if let Symbol::Char(x) = l2 {
        println!("{}", x)
    }

    println!("{}", fibonacci(10));

    #[derive(Debug)]
    struct Point {
        x: u64,
        y: u64,
    }

    impl Point {
        // 构造方法
        fn new(x: u64, y: u64) -> Point {
            Point { x, y }
        }

        fn get_x(&self) -> u64 {
            self.x
        }

        fn set_x(&mut self, x: u64) {
            self.x = x;
        }
    }

    let mut p = Point::new(10, 20);
    println!("{:?}", p);
    println!("{:?}", p.get_x());
    p.set_x(30);
    println!("{:?}", p);

    // 闭包
    let time3 = |n: u32| -> u32 {n*3};
    println!("{:?}", time3(30));
    // move，将环境中的值，移到闭包内部，使用场景：多线程的时候，从主线程当中，移动值到子线程
    // join，除非子线程退出，不然主线程一直等待
    let hello_msg = "hello world";
    thread::spawn(move || {
        println!("{}", hello_msg);
    }).join();

    type Method = fn(u32, u32) -> u32;
    fn cal(method: Method, x:u32, y: u32) -> u32 {
        method (x, y)
    }

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn sub(a: u32, b: u32) -> u32 {
        a - b
    }

    println!("{}", cal(add, 10, 30));

    fn cal2(method: &str) -> Method {
        match method {
            "add" => add,
            "sub" => sub,
            _ => unimplemented!(),
        }
    }

    println!("{}", cal2("add")(10,20));

    //发散函数，永远不会被返回的函数，返回值被标记为！这是一个空类型
    fn foo() -> ! {
        panic!("foo");
    }

    let mut v: Vec<i32> = Vec::new();
    for i in 0..10 {
        v.push(i);
    }
    println!("{:?}", v);

    let mut map = HashMap::new();
    map.insert("foo", 1);
    println!("{:?}", map);
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

//是一门静态编程语言

// 创建变量let关键字
// 变量默认不可变
// 可变变量：变量名称前加mut

// 常量定义：const关键字
// 常量是编译期进行求值的

// 进行加减乘除，如果可能会溢出，最好用overflow相关方法运算

// 元组可以将多种类型的值组合成一个符合类型的通用方法。元祖有固定大小，一旦声明，就不能变化

// super上级模块，self当前模块

// Traits，类似接口