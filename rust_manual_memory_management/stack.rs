/*
As soon as main is called, a stackframe is allocated for it. The local variable a is put on the stack.
The size of the stackframe is determined by the number of local variables and the size of each local variable, which is 32 bits in this case, just enough.
When stack_only is called, a new stackframe is allocated on top of the previous one. The local variable b is put on the stack.
The stackframe for the stack_only function is larger than the stackframe for the main function, because it has two local variables. The variable c is stored on the stack.
After stack_only returns, its stackframe is deallocated, and the stackframe for main is restored, only the stackframe for main is left storing the variable a.
After the main function returns, the stackframe for main is released, and the program exits.
*/
fn main() {
    let a: i32 = 2;
    stack_only(a);
}

fn stack_only(b: i32) {
    let c: i32 = 3;
}
