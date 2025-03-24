# Calculator Application

## Overview
This project is a simple calculator application built using **Rust** and **eframe** for the user interface. The calculator supports basic arithmetic operations such as addition, subtraction, multiplication, division, and modulo. The app also supports parenthesis for grouping operations and ensures proper input validation and error handling.

## Technologies Used

### 1. **Rust**
   The core of the application is written in **Rust**, a systems programming language known for its speed, memory safety, and concurrency capabilities. Rust is used for both the application logic (parsing and evaluating expressions) and for creating a native, efficient executable.

### 2. **eframe**
   **eframe** is a framework for building native desktop applications with **egui**. It provides a simple way to create cross-platform GUI applications in Rust. In this project, **eframe** is used to build the user interface, handling input, button clicks, and displaying results.

### 3. **egui**
   **egui** is a lightweight, portable GUI library for Rust. It is used in this project as part of **eframe** to create the layout, manage the display of buttons, text fields, and handle interactions in the application.

## Features

### 1. **Basic Arithmetic Operations**
   The calculator supports:
   - **Addition (+)**
   - **Subtraction (-)**
   - **Multiplication (x)**
   - **Division (/)**
   - **Modulo (%)**

### 2. **Parenthesis Handling**
   Parenthesis can be used for grouping operations. The application handles nested expressions correctly and ensures proper order of operations.

### 3. **Dot Handling**
   The application allows the use of the decimal point (.) for floating-point numbers, ensuring that multiple dots are not placed in a number and they are properly handled in the expression.

### 4. **Input Validation**
   The calculator validates the user input to ensure that only valid characters are allowed. It checks for mismatched parenthesis and division by zero, providing error messages for invalid operations.

### 5. **Responsive User Interface**
   - The UI is clear, minimal, and intuitive.
   - Buttons are arranged for quick access to numbers, operators, and special symbols.
   - The display area shows the current expression.
   - The UI is designed to be user-friendly, with easy navigation for both beginners and advanced users.

### 6. **Clear and Backspace**
   - The **Clear (C)** button resets the input.
   - The **Backspace** button removes the last entered character.

### 7. **Error Handling**
   If the user enters an invalid expression, such as mismatched parenthesis or division by zero, the app displays error messages like "Invalid expression" or "Division by zero."

## Architecture

### 1. **Main Application (`main.rs`)**
   The entry point for the application. It initializes the `eframe` window and configures basic settings like:
   - **Fixed viewport size** (400x500 pixels)
   - **Centered window** with the **resizable** option disabled

### 2. **Calculator Logic (`calculator.rs`)**
   This module handles the core functionality of the application:
   - **Token Parsing**: The input expression is parsed into tokens (numbers and operators).
   - **Operator Precedence**: Operations are applied according to the correct order (multiplication, division, and modulo take precedence over addition and subtraction).
   - **Error Handling**: The module ensures that invalid expressions (like division by zero or mismatched parenthesis) are caught and handled appropriately.

### 3. **User Interface (`lib.rs`)**
   The UI is built using **eframe** and **egui**, with the following components:
   - **Buttons** for digits, operators, parenthesis, and actions like clear and backspace.
   - **Text Display** to show the current expression.
   - **Layout** optimized for a simple and intuitive interface.

### 4. **State Management**
   The application manages the input state and provides methods to:
   - Add characters to the input string
   - Handle button clicks
   - Compute the result based on the input expression
   - Prevent invalid inputs (e.g., multiple consecutive operators)

## Example Usage

### Basic Operations:
- Enter `5 + 3` and click `=`, the result will be `8`.
- Enter `10 - 2` and click `=`, the result will be `8`.

### Parenthesis and Order of Operations:
- Enter `(3 + 2) x 2` and click `=`, the result will be `10`.
- Enter `2 + (5 x 3)` and click `=`, the result will be `17`.

### Error Handling:
- **Division by Zero**: If you attempt to divide by zero, the application will show "Division by zero".
- **Mismatched Parenthesis**: The app will indicate "Mismatched parenthesis" if there are unmatched parenthesis.

## Conclusion
This calculator application demonstrates how to use **Rust** and the **eframe** framework to build a functional and efficient desktop application. The app is designed to be fast, with error handling, responsive UI, and support for mathematical operations. It highlights the power of Rust for building cross-platform native applications with modern UI capabilities.