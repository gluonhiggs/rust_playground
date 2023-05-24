/* 
create a main function that allocates a variable on a stack and calls another function that allocates another variable on a stack.
*/
fn main() {
    let a: i32 = 2;
    stack_only(a);
}

/* 
create the stack_only function that takes a parameter and allocates a variable on the given stack, and call stack_and_heap function.
*/
fn stack_only(b: i32) {
    let c: i32 = 3;
    stack_and_heap;
}
/* 
create the stack_and_heap function that allocates a variable on a stack and another variable on a heap.
*/

fn stack_and_heap() {
    let d: i32 = 4;
    let e: Box<i32> = Box::new(5);
}