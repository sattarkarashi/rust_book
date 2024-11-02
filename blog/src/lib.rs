pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state:Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pun fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review)
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(Self: Box<Self>) -> Box<dyn State>;
}

trait Draft {}

impl State for Draft {
    fn request_review(self:Box<self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(Self: Box<Self>) -> Box<dyn State> {
        self
    };
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(Self: Box<Self>) -> Box<dyn State>{
        Box::new(Published {})
    };
}

struct Published {}

imple State for Published{
    fn request_review(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(Self: Box<Self>) -> Box<dyn State>{
        self
    };
}

impl State for Draft {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
