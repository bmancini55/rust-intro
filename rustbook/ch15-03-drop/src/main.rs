struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    // These values will automatically be dropped when they go out of
    // scope. When the do go out of scope the Drop trait on the class
    // executes the drop method.
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
    println!("CustomSmartPoints should be dropped.");

    // This is an example of freeing a value by using the drop method
    {
        let c = CustomSmartPointer {
            data: String::from("manual drop"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer has been dropped manually.");
    }
}
