pub struct Post {
    content: String,
    approval_counter: u8,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&mut self) -> &str {
        if self.approval_counter < 2 {
            "Sorry, this needs one more approval before it can be published."
        } else {
            &self.content
        }
    }

    pub fn approve(mut self) -> Post {
        if self.approval_counter < 2 {
            self.approval_counter += 1;
        }
        self
    }

    pub fn reject_back_to_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
            approval_counter: 1,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}