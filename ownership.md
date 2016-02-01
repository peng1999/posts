# Rust: Safe with efficiency 

> I'm writing this post in Onenote. You can visit my latest work [there](https://onedrive.live.com/redir?page=view&resid=AC248F4C0C749874!1627&authkey=!AIF2G_Q_9-XVKKI). 

## Overview 
Well, there have been a lot of great articles about rust's ownership system, including the [Rust Book][2] and its [future iteration][3], which contains some chapters about Ownership. However, most of them focus on the rules we should follow. This post is aimed to talk about the ownership in a higher level - Not only the rules, but also how it works. 
### Related work, and why Rust is better 
The new languages which have released recent years can all automatically manage memory. (Even C++ will have a lifetime system soon.[1]) It seems that people can't tolerate manage memory themselves  anymore. Different languages have different ways to make memory safety, so let's have a brief look: 
- **C#, Java and other language based on CLR or JVM:** 
    They use the garbage collector based on mark-sweep algorithm. This will cause some runtime cost, but programmers needn't think about memory anymore. 
- **Swift, Delphi and some other languages:** 
    They use reference count. This also has runtime overhead, but the cost is smaller than mark-sweep collector. 
- **C++:** 
    Many years ago, C++ manage its memory by explicitly allocate and free. This is the fastest way to manage memory, but it's caused a lot of trouble, some of which I will talk about later. Nowadays, the ISO C++ have developed a "[lifetime System][1]" to eliminate leaks and dangling for pointers. 

Rust is a system language, which means it should make the code as quick as possible. So it can't tolerate unnecessary runtime overhead. The only way seems to be "zero abstractions" like C++. 

It has developed a ownership system to prevent all memory problems and achieve some great features. 
- **Easy to analyze** 
    the rule of the ownership system is enough simple to learn and memorize, so most of the problem borrow checker have found can be fix quickly. 
- **More general** 
    The C++ lifetime is simply aimed to eliminate leaks and dangling, the garbage collector of C#, etc. can't deal with other resource like socket or files, while Rust deal with any kinds of resource with ownership system. 
- **Work well in both synchronous and asynchronous code** 
    Asynchronous safety is another serious problem. Surprisingly, ownership can work in both synchronous and asynchronous context. 
- **Messages in type** 
    Rust has a powerful strong type system. Thus, we're able to put some messages in type. (like lifetime message, nullable semantic, etc.) It helps a lot. 
 
------------- 
For a program language for real world projects, there can't be whole-program analysis. So the ownership system makes sure that the code in every function is safe. Let's talk about safety in function body first. 

## Safe within function body 
### Problem of C++ 
Let's start with a common problem in C++: 

 

 

[1]: https://github.com/isocpp/CppCoreGuidelines/blob/master/docs/Lifetimes%20I%20and%20II%20-%20v0.9.1.pdf (Lifetime I and II) 
[2]: http://doc.rust-lang.org/stable/book (the Rust Book) 
[3]: http://rust-lang.github.io/book/ownership.html (future iteration of the Rust Book) 