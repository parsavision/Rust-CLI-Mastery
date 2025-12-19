# Rust CLI Mastery

A comprehensive guide to building command-line applications in Rust, with practical exercises and real-world examples.

## Table of Contents
- [Introduction](#introduction)
- [Exercises](#exercises)
- [Resources](#resources)

## Introduction

This repository contains a collection of exercises designed to help you master the art of building robust, efficient command-line applications in Rust. Each exercise builds upon the previous one, introducing new concepts and best practices.

## Exercises

### Exercise 1: Hello CLI

**Goal:** Create your first command-line application in Rust.

**Task:** Build a simple program that accepts a name as input and prints a personalized greeting.

**Concept:** Basic program structure, function definitions, and console I/O in Rust.

**Help/Notes:** 
- Use `std::io` for reading input
- Remember to handle the newline character when reading input
- The `trim()` method will be useful

---

### Exercise 2: Temperature Converter

**Goal:** Build a tool that converts temperatures between Celsius and Fahrenheit.

**Task:** Create a program that accepts a temperature value and unit, then converts it to the other unit and displays the result.

**Concept:** Parsing user input, conditional logic, and mathematical operations in Rust.

**Help/Notes:**
- Parse the input string to extract temperature and unit
- Use pattern matching for handling different units
- Formula: F = (C × 9/5) + 32

---

### Exercise 3: Word Counter

**Goal:** Develop a utility that counts words in a given text or file.

**Task:** Write a program that reads text input and provides statistics: total words, unique words, and word frequency.

**Concept:** String manipulation, collections (HashMap), and iterators in Rust.

**Help/Notes:**
- Use `HashMap` to track word frequencies
- Convert words to lowercase for accurate counting
- The `split_whitespace()` iterator is helpful here

---

### Exercise 4: File Explorer

**Goal:** Create a tool that explores and displays directory structures.

**Task:** Build a program that takes a directory path and recursively lists all files and subdirectories with their sizes.

**Concept:** File system operations, recursion, and error handling in Rust.

**Help/Notes:**
- Use `std::fs` module for file operations
- Implement proper error handling with `Result` types
- Consider using recursion for directory traversal

---

### Exercise 5: JSON Parser

**Goal:** Build a simple JSON parser or validator.

**Task:** Create a program that reads a JSON string and validates its syntax, reporting any errors found.

**Concept:** Parsing complex structures, error handling, and trait implementations in Rust.

**Help/Notes:**
- Consider using the `serde_json` crate for simplicity
- Implement basic validation without external libraries as an advanced challenge
- Focus on clear error messages

---

### Exercise 6: Task Manager

**Goal:** Develop a command-line task management application.

**Task:** Build a program that allows users to add, remove, list, and mark tasks as complete, with data persistence using a file.

**Concept:** Data structures, file I/O, serialization, and application design in Rust.

**Help/Notes:**
- Use `serde` and `serde_json` for serialization
- Design a clear menu system for user interaction
- Implement file locking to prevent data corruption

---

### Exercise 7: HTTP Client

**Goal:** Create a simple HTTP client application.

**Task:** Build a program that can perform GET and POST requests to web servers and display the response.

**Concept:** Networking, async programming basics, and dependency management in Rust.

**Help/Notes:**
- Use the `reqwest` crate for HTTP operations
- Consider implementing async/await for better performance
- Handle timeout and error scenarios gracefully

---

### Exercise 8: Configuration Manager

**Goal:** Build a configuration file parser and manager.

**Task:** Create a program that reads, validates, and manages application configuration from TOML or YAML files.

**Concept:** File parsing, error handling, configuration patterns, and environment variable handling in Rust.

**Help/Notes:**
- Use `toml` or `serde_yaml` for parsing
- Support environment variable overrides
- Provide helpful error messages for invalid configurations

---

### Exercise 9: Git Clone Simulator

**Goal:** Simulate basic Git repository operations.

**Task:** Build a simplified version of Git that can initialize repositories, commit changes, and display commit history.

**Concept:** Advanced data structures, version control concepts, and complex application design in Rust.

**Help/Notes:**
- Design an efficient commit storage system
- Implement branching (as an advanced feature)
- Use `.git` directory structure similar to real Git

---

### Exercise 10: Plugin System

**Goal:** Create a plugin architecture for extending application functionality.

**Task:** Develop a system that loads and executes plugins from shared libraries (.so, .dll, .dylib), allowing dynamic extension of application features.

**Concept:** Dynamic loading, FFI (Foreign Function Interface), advanced macro usage, and system-level programming in Rust.

**Help/Notes:**
- Use `libloading` crate for dynamic library loading
- Define a stable plugin interface with traits
- Ensure plugin compatibility and safety considerations

---

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Community Crates](https://crates.io/)

---

**Last Updated:** 2025-12-19

Happy coding! 🦀
