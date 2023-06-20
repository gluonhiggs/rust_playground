## Prerequisites

### Stack

A stack has the following characteristics:

- It is a special region in the computer process memory that stores temporary variables created by a function.
- For a function called, a new stack frame is created and pushed to the top of the current stack. (This is usually imagined as LI in LIFO - Last In First Out)
- The size of every variable on the stack has to be known at compile time.
- When a function exits, the stack frame is released and the stack pointer is set back to the previous value. (This is usually imagined as FO in LIFO - Last In First Out)

### Heap

A heap has the following characteristics:

- It is a region of the process memory that is not managed automatically for you.
- It has no size limit.
- It is accessible globally in the program.
- Heap allocations are expensive and should be used sparingly.


### Smartpointer

- A smart pointer is a class that wraps a raw pointer, to manage the lifetime of the pointer.
- The smart pointer automatically deletes the object when the smart pointer goes out of scope.

  