# Hadron Launcher Project (`GEMINI.md`)

This document provides a comprehensive overview of the Hadron project for future development and maintenance.

## 1. Project Overview

**Purpose:**
Hadron Launcher is a graphical user interface (GUI) application for Linux, designed to act as a simple "Wrapper to Proton." It allows users to add, manage, and launch games (likely Windows games) using Valve's Proton compatibility layer.

**Technology:**
- **Language:** Rust (2024 Edition)
- **GUI Framework:** [`slint`](https://slint-ui.com/), a declarative UI toolkit for Rust.
- **Data Serialization:** Game metadata and settings are persisted using the TOML format.

**Architecture:**
- **`main.rs`**: The application entry point, responsible for initializing and running the Slint application.
- **`app.rs`**: The core of the application logic, handling data loading, saving, and UI callbacks.
- **`models.rs`**: Defines the primary data structures: `Game` (containing name, paths to the executable, cover art, and Wine prefix) and `Settings` (containing the path to Proton).
- **`utils.rs`**: A module for utility functions.
- **`ui/`**: This directory contains the Slint UI definitions for the application, separated into components.

## 2. Building and Running

The project uses the standard Rust build tool, Cargo.

- **Build the project:**
  ```bash
  cargo build
  ```

- **Run the application:**
  ```bash
  cargo run
  ```

- **Run tests:**
  ```bash
  cargo test
  ```

## 3. Development Conventions

- **Modularity:** The codebase is structured into modules based on functionality (`app`, `models`, `utils`, `ui`). This separation of concerns should be maintained.
- **State Management:** The global application state is managed in `app.rs`.
- **UI and Styling:** The UI is defined declaratively using the `.slint` language. The "cosmic" style is used, as defined in `build.rs`.
- **Data Persistence:** All user data is stored in a single TOML file. Changes to the `Game` or `Settings` structs in `models.rs` must be compatible with the serialization/deserialization logic in `app.rs`.
