// This Rust code is designed to demonstrate common programming concepts and data structures.
// It includes basic operations such as variable declarations, function definitions,
// loops, conditionals, etc., along with some example code for managing a stack.

/// A simple data structure to hold elements.
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Adds an element to the top of the stack.
    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Removes and returns the top element from the stack without removing it.
    fn pop(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        let last_value = &self.items[self.items.len() - 1];
        self.items.pop();
        Some(last_value)
    }

    /// Returns the top element from the stack without removing it.
    fn peek(&self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        self.items[self.items.len() - 1]
    }

    /// Checks if the stack is empty.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Prints the contents of the stack to the console.
    fn print_contents(&mut self) {
        for item in &self.items {
            println!("{}", *item);
        }
    }
}

fn main() {
    // Example usage
    let mut stack = Stack::new();

    // Add elements to the stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Print the top element
    println!("Top element: {:?}", stack.peek());

    // Get and print the current top element (will not remove it)
    if let Some(top) = &stack.peek() {
        println!("Current top: {}", *top);
    }

    // Remove and print the last element from the stack
    if let Some(last_value) = &stack.pop() {
        println!("Last value popped: {}", *last_value);
    } else {
        println!("Stack is empty.");
    }

    // Check if the stack is empty
    assert_eq!(stack.is_empty(), true);

    // Print all contents of the stack
    for item in &mut stack.items {
        println!("{}", *item);
    }
}
