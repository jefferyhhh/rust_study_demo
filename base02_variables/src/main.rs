fn main() {
    let mut x = 5;//通过 `mut` 关键字让变量变为可变的
    let a = 2i32;//可在字面量中加入类型注解，可以包含下划线
    let _b = 100;//下划线忽略未使用的变量
    const MAX_POINTS: u32 = 100_000;//常量，类型必须标注，约定是全部字母都使用大写，并使用下划线分隔单词
    println!("The value of x is: {}", x);//5
    x = 6;
    println!("The value of x is: {}", x);//6
    println!("{}", a);//2
    println!("{}", MAX_POINTS);//100000
}