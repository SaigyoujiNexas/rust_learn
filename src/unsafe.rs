extern "C"{
fn abs(input:i32) -> i32;
}

//五大能力
//解引用裸指针
//调用不安全函数
//访问或修改可变静态变量
//实现不安全trait
//访问union字段

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe { *r2 = 3 };
    println!("{num}");

    let num2 = unsafe {
        *r1;
    };

    let address = 0x012345usize;
    let r = address as *const i32;
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);

}
