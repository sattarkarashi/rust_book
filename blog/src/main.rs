use blog::Post;

fn main(){

    // Implementing a post blog using state pattern
    
    let mut post = Post::new();
    post.add_text("I like writing.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I like writing.", post.content());

}