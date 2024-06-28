# Linked list (Written in Rust)
I'm new to Rust and trying to learn. During a job interview at one of my previous jobs,
one of the interview questions was to implement a linked list class (using Python) 
with the following methods:
- `def get(self, index: int) -> T`
- `def prepend(self, value: T) -> None`
- `def append(self, value: T) -> None`
- `def insert(self, index: int, value: T)`
- `def remove(self, index: int) -> T`

I got the job and subsequently got the oppertunity to ask other interviewees the same question.
Rust's ownership rules make this exercise a lot harder than other languages. Much harder still 
than languages with 'manual' memory management like C, I think. I was unable to write this in 
one sitting, mostly because I'm still not very familiar with Rust's borrow checking rules. I 
believe this was a good exercise. I would encourage anyone trying to learn Rust to try it for 
themselves.

## Some notes
I am aware of the [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
book. I read part of it while trying to write my own implementation. I learned quite a bit from it. It has several 
implementations of linked lists: Starting with a simple singly linked list, a doubly linked lists, lists that use 
different types of pointers, thread-safe lists, etc. Although it seems to me that all of these lists have a
'stack' or 'queue'-like interface. It's only possible to modify the lists by adding items at the beginning or end
of the list. That's completely fine, but I need to implement methods to remove or insert items from/to the middle
of the list as well. 

An interesting thing I noticed is that AI chatbots (Copilot in my case) have trouble writing Rust code that 
adheres to the borrowing rules as well. It would make the very same mistakes I made: i.e. ending up with 
multiple mutable references to the same thing. I even tried informing it that it had made a mistake. It would
apologize and try something else, but it could not generate code that would compile.
