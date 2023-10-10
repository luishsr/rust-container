# rust-container
A minimalist Application Container in Rust for learning purposes

Simple Container System in Rust 📦🦀

A minimalistic container system built with Rust. This project provides a basic implementation of container features, demonstrating how applications can be isolated and run in a separate environment. Perfect for those seeking to understand the foundational principles behind container technology.
Features 🌟

  - Isolated Environment: Uses Linux namespaces to achieve process and filesystem isolation.
  - Simple CLI: Deploy and run applications with ease.
  - Statically Linked Application Support: Designed to run applications without external dependencies for maximum simplicity.

Getting Started 🚀

Prerequisites

  - Rust (latest stable version)
  - Cargo

Installing the 'libc' package on Linux OS

Before getting started with the crate, you'll want to make sure you've got the libc package set up on your system.

For Debian-based systems like Ubuntu, the C library is provided by the libc6 package, and the header files are in the libc6-dev package. You can install it with:
     
     sudo apt update
     sudo apt install libc6-dev

Installation

   Clone the repository:

    git clone https://github.com/luishsr/rust_container.git
    cd simple-container-rust

   Build the project:

    cargo build --release

For Running the Project, I recommend starting with the containerclient example we built in the Article, as it does not require libraries' dependencies (as it is statically linked). But you can try it with any application, as long as you copy the libs dependencies of your application along with its binaries.

Using the sample containerclient for testing the Container deployment:

   CD into the containerclient project:

    cd containerclient
   

   Install the MUSL Target:
     
     rustup target add x86_64-unknown-linux-musl
   
   Build the project manually using the :

     cargo build --release --target x86_64-unknown-linux-musl  

   This will produce a statically linked binary in the target/x86_64-unknown-linux-musl/release/ directory.  

Deploying and Running ContainerClient in Our Simple Container

   Deploying the ContainerClient Application:

    sudo ./rustcontainer deploy ./containerclient

   Running the ContainerClient Application Inside the Container:

    sudo ./rustcontainer run ./containerclient

If all went well, you should see:

    Hello from the container!

And that's it! With these simple steps, you've deployed and executed your containerclient application inside our minimalist container environment. Enjoy experimenting with it!







