# 🎵 Kreck

**Kreck** is a desktop application that lets you control sound and music in Discord bots from your phone or other devices. It integrates with **Kenku FM**, an application that allows seamless connection with music bots on Discord.

## 🚀 Features

- 🎶 **Remote Sound Control**: Manage Discord bots with your smartphone or tablet.
- 🖥️ **Cross-Platform**: Available for Linux, macOS, and Windows.
- 🌐 **Web-Based Interface**: Accessible, simple and modern, any device can run.
- ⚡ **Built with Tauri**: Efficient and lightweight desktop application.

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)**: The main programming language used for backend logic.
- **[Tauri](https://tauri.app/)**: Framework for building lightweight and cross-platform desktop applications.
- **[Rocket](https://rocket.rs/)**: A web framework written in Rust, serving the web pages accessible from mobile devices.
- **HTML + CSS + JavaScript**: For building the user interface and frontend logic.

## 💻 Installation

### Prerequisites

For up-to-date information on dependencies for each platform (Linux, macOS, and Windows), please visit the official Tauri [documentation](https://tauri.app/start/prerequisites/).

### Build & Run

1. Clone the repository:
   ```bash
   git clone https://github.com/Lucas-BRT/Kreck.git
   cd Kreck
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run tauri dev
   ```

### Build for Production

To create a production build:

```bash
npm run tauri build
```

## 🧑‍💻 Contributing

1. Fork the repository.
2. Create a new branch for your feature:
   ```bash
   git checkout -b feature-name
   ```
3. Make your changes and commit:
   ```bash
   git commit -m "Add a new feature"
   ```
4. Push to your fork:
   ```bash
   git push origin feature-name
   ```
5. Open a Pull Request.

## 🖥️ Build on Different Platforms

- **Linux**: Follow the prerequisites and installation instructions on the Tauri website.
- **macOS & Windows**: Simply ensure **Rust** and **Node.js** are installed, and run the build commands.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
