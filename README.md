# Project description

This program does nothing; it is just one of the two blog post backend model examples in the [rust lang book with the three extra suggestions implemented](https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#trade-offs-of-the-state-pattern). This repo is a modification of the example written in conventional Rust. 

# Notes on the project

The first and third suggestions are trivial. The second suggestion is a bit confusing and annoying, so I cheated (a bit) by moving the approval_counter field from the PendingReviewPost struct to the Post struct. In this way, the 'blog post' can be approved from PendingReviewPost to Post, but will still require 'two approvals' before its contents can be published. 

I haven't seen anyone else do it this way. There have been some discussion on the problem and how to solve it (please see [here](https://users.rust-lang.org/t/solving-an-an-exercise-from-the-chapter-17-of-trpl/69679) and [here](https://users.rust-lang.org/t/chapter-17-3-multiple-approvals-with-type-system/62290)). It is possible to solve it by use of Options/Enums/Result/smart pointers like Box, but they kick the problem of determining the type return down the line, usually to the client/main. One could also use a crate called [Either](https://crates.io/crates/either), and it certainly seems very useful, but I didn't want to use a crate for this one. 

Another possible solution is to use multiple structs to differentiate between having zero/one/two approval(s) and move the blog post along them as it gets more approval, but I didn't like it for several reasons.
a. The idea of having several approval structs is quite similar to the different struct states in the other blog post example that has a more OOP pattern. That is not a bad thing, but I wanted this example to be different from that and be more 'Rust-like'.
b. Since each approval counter needs one struct, a design problem can occur if 10 or more approvals are suddenly needed.
c. I wanted my solution to be relatively simple and clean.

All that to say that I eventually settled on the idea to move the approval counter from the PendingReviewPost struct to the Post struct. It also handles the problem of potentially needing more approvals down the line. 

# Problems and potential tradeoffs

A PendingReviewPost struct is upgraded to Post struct even if isn't fully approved. This could be a design flaw, but there's currently no way to see a blog post's content unless it has enough approvals. I admit that it is a weird design. 