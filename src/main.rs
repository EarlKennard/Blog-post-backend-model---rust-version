use implementing_an_oop_design_2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let mut post = post.approve();

    assert_eq!("Sorry, this needs one more approval before it can be published.", post.content());

    let mut post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
