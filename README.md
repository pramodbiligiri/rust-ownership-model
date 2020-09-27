Memory semantics in Rust
==================================

This project contains code that demonstrates the various memory semantics of the assignment operator in Rust.
The table and state diagrams below depict the same in a visual manner.

There is a blog post that explains the concepts behind this code: [Learn Rust: Assignment and Memory Semantics](https://www.pramodb.com/index.php/2020/09/27/learn-rust-assignment-and-memory-semantics/) 

Table of Assignment related memory semantics
-------------------------------------

| Assign From | Assign To | Effect on source variable | 
| ---------:| -----------:| ----------------:|
| Read-Only Owner  | Read-Only Owner  | Permanently invalidated |
| Read-Only Owner  | Read-Write Owner | Permanently invalidated |
| Read-Write Owner | Read-Write Owner | Permanently invalidated |
| Read-Write Owner | Read-Only Owner  | Permanently invalidated |
| Read-Only Owner  | Read-Only Ref    | Becomes RO, Non-Movable till Dest can't be expired |
| Read-Write Owner | Read-Write Ref   | Becomes unusable till dest can't be expired        | 
| Read-Write Owner | Read-Only Ref    | Becomes RO, Non-Movable till dest can't be expired | 

Transition Diagram for Assignment related memory semantics
------------------
![Transition Diagram](rust-memory-2.svg)

TODO
----
- Add Copyable types to this list?