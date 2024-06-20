
fn main() {
    let a =2;
    println!("Main Fn: a= {a}");
    unsafe { stack_only(2); }
    println!("After stack_only in main")
}

unsafe fn stack_only(b :i32) {
    let c = 3;
    println!("Fn stack_only: b={b} c={c}");
    stack_and_heap();
    println!("After stack_and_heap in stack_only")
}

unsafe fn stack_and_heap(){
    let d = 5;
    let e: *mut i32 = &mut 7;
    println!("Fn stack_and_heap: d={d}");
    println!("e = {:?}", e);
    println!("&e = {:?}", &e);
    println!("*e = {:?}", *e);
println!("Dropping e: {:?}", drop(e));
}