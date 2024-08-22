
---

# Cooking Wiki - Rust Powered Web Application

Welcome to **Cooking Wiki**, a lightweight web application built with Rust, Actix-web, and Askama, designed to help you organize and display your favorite cooking recipes beautifully. This simple tool provides a user-friendly interface where you can showcase your meals, complete with ingredients, steps, and images, all within a modern, responsive web interface.

## Features

- **Simple & Lightweight:** Built with Rust, ensuring high performance, stability, and safety.
- **Dynamic Content:** Add recipes with ease, each complete with a title, ingredients, steps, and images.
- **Responsive Design:** The interface is clean, modern, and responsive, making it ideal for mobile and desktop users.
- **Fast & Reliable:** Utilizing Actix-web, a fast and secure web framework, the Cooking Wiki loads quickly and handles requests efficiently.

## Video Demo

[Include a link to a video demo here, showcasing the web application in action.]

## Why Use Cooking Wiki?

Cooking Wiki is ideal for users who want a personal recipe manager that is fast, easy to use, and visually appealing. The application leverages the power of Rust for performance and security, making it a reliable choice for personal or even small-scale professional use. Users looking for a simple, customizable, and efficient recipe catalog will find this tool incredibly valuable.

## Installation & Setup

To run this application, follow these instructions:

### Prerequisites

1. **Rust**: Ensure you have Rust installed. You can install Rust via `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Gitpod (Optional)**: If you want to run this project in a cloud-based development environment, set up a [Gitpod workspace](https://www.gitpod.io/).

### Building the Project

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/cooking-wiki.git
   cd cooking-wiki
   ```

2. **Install Dependencies:**
   The dependencies are managed by Cargo, Rust’s package manager. Your `Cargo.toml` should look like this:
   ```toml
   [package]
   name = "rust_time_tracker"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   actix-web = "4.9.0"
   askama = "0.12.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_derive = "1.0"
   serde_json = "1"
   ```

3. **Run the Application:**
   ```bash
   cargo run
   ```

4. **Access the Application:**
   After running the application, visit `http://127.0.0.1:8080` in your web browser to interact with the Cooking Wiki.

## Customization

To add more recipes, modify the `src/main.rs` file. Recipes are stored as a list of `Recipe` structs with fields for the name, ingredients, steps, and an image URL. Feel free to add or modify entries as needed.

The HTML layout is found in the `templates/index.html` file, which uses the Askama templating engine to dynamically render content.

## Conclusion

Cooking Wiki is a powerful yet simple tool that provides users with a personalized way to organize their recipes. Built on top of Rust’s ecosystem, this tool offers the performance and stability needed to handle your favorite meals. Whether you’re a hobbyist chef or just looking for a place to store your culinary ideas, Cooking Wiki is the tool for you!

