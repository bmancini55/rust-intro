mod blog;
mod blog2;

fn main() {
    // This first example uses a traditional OOP implementation of the
    // state pattern. The implementation has some complexities due to
    // Rust's ownership rules.
    {
        let mut post = blog::Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    // Alternatively, we can store state in types and transition bewteen
    // types. This allows us to use rust's variable shadowing as the
    // underlying type changes.
    {
        let mut post = blog2::Post::new();
        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.contents());
    }
}
