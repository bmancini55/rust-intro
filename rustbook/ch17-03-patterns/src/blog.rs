pub struct Post {
    /// Private member that contains the current state of the Post
    state: Option<Box<dyn State>>,

    /// Private contents that may or may not be revealed based on the
    /// state of the current Post
    content: String,
}

impl Post {
    /// Constructs a new post with default state of Draft and empty
    /// contents, both of which are private.
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// Obtains the contents from the state
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().contents(self)
    }

    /// Adds text to the post by appending it to the current content
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Mutates the state by taking ownership of the state using `take`
    /// otherwise it would require a copy, which state doesn't implement
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    /// Mutates the state by taking ownership of the state using `take`
    /// otherwise it would require a copy, which state doesn't implement
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

/// Defines the behavior shared by different post states  which will be
/// implemented by the different state structs.
trait State {
    /// Defines a `request_review` method that can only be called when
    /// a `Box` is holding the type.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    /// Defines a `approve` method that can only be called when a `Box`
    /// is holding the type.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// Returns the contents from the supplied post based on the
    /// implementation of the state. The default implementation returns
    /// no contents
    fn contents<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/// Defines a draft state which does not return the state
struct Draft {}
impl State for Draft {
    /// Transition from Draft to PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    /// No transition
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

/// Defines the pending review state
struct PendingReview {}
impl State for PendingReview {
    // No transition
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Transition from Pending Review to Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

/// Defines the approved state that returns contents
struct Published {}
impl State for Published {
    // No transition
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // No transition
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Create a slice from the contents in the post
    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
