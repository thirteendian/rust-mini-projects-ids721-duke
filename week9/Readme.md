# Week9 mini project

This is a mini project for Chat Application: a real-time chat application using web sockets. This project uses the actix-web crate for building the server and the tungstenite crate for handling web sockets.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Dependencies

- actix-web
- actix-web-actors
- actix
- tungstenite
- futures
## Project Structure

The project consists of the following files:

- `src/main.rs`: Contains the main function that starts the HTTP server and registers the WebSocket route.
- `src/chat.rs`: Contains the core chat functionality, including the chat server, chat session, and message handling.

## Running the Chat Application
1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build` to compile the project.
4. Run `cargo run` to start the server.
5. Connect to the WebSocket server at `ws://127.0.0.1:8080/ws/` using a WebSocket client or a JavaScript WebSocket implementation in the browser.

## Additional Features
This example provides a minimal chat application. To enhance the application for production use, consider adding the following features:

- User authentication
- Message history
- User management
- Private messaging
