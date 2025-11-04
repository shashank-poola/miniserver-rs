# Simple Multi-Threaded Web Server in Rust

A lightweight, multi-threaded web server built from scratch using **Rust**.  
This project demonstrates systems programming, concurrency, and basic HTTP handling without relying on external frameworks.

---

## Overview

This project is a simple implementation of a web server that:
- Accepts incoming HTTP requests  
- Responds with static HTML files  
- Uses a custom-built thread pool for concurrency  
- Gracefully shuts down after processing a set number of requests  

It is inspired by the *“Building a Multithreaded Web Server”* project from *The Rust Programming Language (The Book)*, extended with better structure and documentation.

---

## Features

- Built entirely with standard Rust libraries  
- Custom **thread pool** for efficient concurrency  
- Handles **multiple client requests** simultaneously  
- Serves basic static files (HTML, CSS, JS)  
- Graceful shutdown support  
- Modular and clean codebase  


## How It Works

1. The server binds to a specified IP address and port.  
2. It listens for incoming TCP connections.  
3. Each connection is processed by a worker thread from the pool.  
4. The worker parses the HTTP request and returns an appropriate response.  
5. The server gracefully shuts down after handling a defined number of requests.

---

## Installation and Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Steps

```bash
# Clone the repository
git clone https://github.com/shashank-poola/miniserver-rs.git

# Navigate into the project directory
cd miniserver-rs

# Build the project
cargo build

# Run the web server
cargo run


