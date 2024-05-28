# JavaScript, Rust, and Vertical Scaling with Node.js

### This repository demonstrates the differences in concurrency and parallelism between JavaScript, Rust, and Node.js with the Cluster module.

## Table of Contents:-

#### - Single-Threaded JavaScript
#### - Multithreaded Rust
#### - Vertical Scaling with Node.js Cluster
#### - Conclusion

## Single-Threaded JavaScript :- 

#### JavaScript is a single-threaded language, which means that it can only execute one task at a time. This is demonstrated in the index.ts file in the src folder, which runs an infinite loop. Since JavaScript is single-threaded, this loop will be executed on a single CPU core, and you cannot parallelize the task to run it on multiple CPU cores.

## Multithreaded Rust :-

#### In contrast, languages like Rust are multithreaded, which means they can take advantage of multiple CPU cores to execute tasks concurrently. The main.rs file in the src folder demonstrates this by using the thread::spawn function to create multiple threads and execute tasks in parallel.

## Vertical Scaling with Node.js Cluster:-

#### While JavaScript is single-threaded, the Node.js runtime provides a way to scale your application vertically by running multiple processes in different CPU cores. The vertical scaling nodejs application folder in this repository contains an example of an Express server that uses the Cluster module to achieve this.

#### The Cluster module in Node.js allows you to create child processes that can share server ports, enabling your application to take advantage of multiple CPU cores. This can be particularly useful when you have a CPU-bound application that cannot be easily parallelized, as it allows you to distribute the workload across multiple processes.

## Conclusion:-

#### This repository showcases the differences in concurrency and parallelism between JavaScript, Rust, and Node.js with the Cluster module. It highlights the single-threaded nature of JavaScript, the multithreaded capabilities of Rust, and the vertical scaling approach provided by the Cluster module in Node.js.
#### By understanding these concepts, you can make informed decisions about the best tools and approaches to use for your specific use case, whether it's a CPU-bound task, a highly concurrent application, or a need for vertical scaling.

