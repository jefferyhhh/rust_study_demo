fn main() {
    let a = 10;
    let b:i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b),add(c,d));
    print!("(a+b)+(c+d)={}",e);
}

fn add(i:i32,j:i32) ->i32{//在定义函数时，类型声明是必须的
    //函数会返回最后一个表达式的求值结果，return不是必需的
    i + j
}