fn main(){
    let a:i32 = 2;
    stack_only(a);
}

fn stack_only(b:i32)-> i32{
    let c:i32 = 3;
    stack_and_heap()
}

fn stack_and_heap(){
    let d:i32 = 4;
    // add a smart pointer to the heap
    let e:Box<i32> = Box::new(5);
}