use implementing_an_object_oriented_design_pattern_2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post: Post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
