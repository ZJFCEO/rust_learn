use std::mem;
const A: i32 = 10;

fn get_number() -> i32 {
    42
}

fn main() {
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
    let jack = Person{
        name: String::from("Jack"),
        age: 6,
    };
    println!("{} {}", jack.name, jack.age);
    println!("{:?}", jack);

    struct Unit;
    let unit = Unit;

    enum IPAddr {
        IPV4(u8, u8, u8, u8),
        IPV6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    }

    let localhost: IPAddr = IPAddr::IPV4(127, 0, 0, 1);
    match localhost {
        IPAddr::IPV4(a,b,c,d)=>{
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
}

//是一门静态编程语言

// 创建变量let关键字
// 变量默认不可变
// 可变变量：变量名称前加mut

// 常量定义：const关键字
// 常量是编译期进行求值的

// 进行加减乘除，如果可能会溢出，最好用overflow相关方法运算

// 元组可以将多种类型的值组合成一个符合类型的通用方法。元祖有固定大小，一旦声明，就不能变化
