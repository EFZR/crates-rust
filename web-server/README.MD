# Asynchronous Web Server made with Rust 🦀

In this project, we've developed a simple asynchronous web server using only the Rust standard library.

Despite its simplicity, the program is quite complex. It utilizes a Thread Pool to handle multiple requests concurrently. This is achieved using the `std::sync::mpsc` module for inter-thread communication, `std::thread` for thread creation, and `std::sync::Arc` and `std::sync::Mutex` to share the `mspc` receiver across threads.

>**Note:** As a developer and a Rust enthusiast, I must admit that some aspects of the code are still beyond my current understanding. However, I am actively learning comprehend it fully.

This project serves as an excellent learning tool for understanding both Rust and the workings of a web server. While it may not be the ideal method for creating a web server, it certainly provides valuable insights into the underlying mechanics.

For more detailed information about this server, please refer to the following link: <https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html>. This resource, part of the Rust Book, is an excellent guide for learning Rust.
