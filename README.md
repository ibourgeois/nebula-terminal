# Nebula Terminal

A modern, cross-platform terminal application built with **Tauri**, **NuxtJS**, and **TailwindCSS**. Nebula Terminal combines the power of Rust for system-level operations with the flexibility of Vue.js for a beautiful, responsive user interface.

## 🚀 Tech Stack

- **Backend**: Rust with Tauri 2.6.0
- **Frontend**: NuxtJS 3.17.5 (Vue.js 3)
- **Styling**: TailwindCSS 4.1.10
- **Build Tool**: Vite
- **Package Manager**: npm

## ✨ Features

- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Modern UI**: Beautiful gradient design with glassmorphism effects
- **Fast Performance**: Rust backend for system operations
- **Responsive Design**: TailwindCSS-powered responsive interface
- **Hot Reload**: Instant development feedback with NuxtJS HMR
- **Type Safety**: Full TypeScript support

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **npm** or **yarn**
- **Git**

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing Node.js

Visit [nodejs.org](https://nodejs.org/) or use a version manager like `nvm`.

## 📦 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/ibourgeois/nebula-terminal.git
   cd nebula-terminal
   ```

2. **Install dependencies**
   ```bash
   # Install Node.js dependencies
   npm install
   
   # Install Rust dependencies (automatic on first build)
   cd src-tauri
   cargo build
   cd ..
   ```

## 🚀 Development

### Starting the Development Server

```bash
# Start NuxtJS dev server only
npm run dev

# Start Tauri with NuxtJS integration
cd src-tauri
cargo tauri dev
```

The application will be available at:
- **Web**: http://localhost:3000
- **Desktop**: Tauri window will open automatically

### Development Workflow

1. **Frontend Changes**: Edit Vue components in the root directory
2. **Backend Changes**: Edit Rust code in `src-tauri/src/`
3. **Styling**: Use TailwindCSS classes or edit `assets/css/tailwind.css`
4. **Configuration**: Update `nuxt.config.ts` or `src-tauri/tauri.conf.json`

## 🏗️ Building

### Web Build

```bash
# Build for production
npm run build

# Preview production build
npm run preview
```

### Desktop Build

```bash
# Build desktop application
cd src-tauri
cargo tauri build
```

The built application will be available in `src-tauri/target/release/`.

## 🧪 Testing

### Frontend Tests

```bash
# Run NuxtJS tests (when implemented)
npm run test
```

### Backend Tests

```bash
# Run Rust unit tests
cd src-tauri
cargo test
```

## 📁 Project Structure

```
nebula/
├── assets/
│   └── css/
│       └── tailwind.css          # TailwindCSS styles
├── components/                   # Vue components
├── layouts/                      # NuxtJS layouts
├── pages/                        # NuxtJS pages
├── public/                       # Static assets
├── src-tauri/                    # Tauri backend
│   ├── src/
│   │   ├── lib.rs               # Rust library code
│   │   └── main.rs              # Tauri main entry
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── app.vue                      # Main Vue component
├── nuxt.config.ts               # NuxtJS configuration
├── tailwind.config.js           # TailwindCSS configuration
├── package.json                 # Node.js dependencies
└── README.md                    # This file
```

## 🎨 Styling

The project uses **TailwindCSS v4.1** for styling:

- **Configuration**: `tailwind.config.js`
- **Custom Styles**: `assets/css/tailwind.css`
- **Design System**: Modern gradients, glassmorphism, responsive design

### Key Design Features

- **Gradient Background**: Blue to purple to indigo
- **Glassmorphism**: Semi-transparent containers with backdrop blur
- **Responsive Typography**: Mobile-first design approach
- **Dark Mode Ready**: Compatible with system dark mode

## 🔧 Configuration

### NuxtJS Configuration

Edit `nuxt.config.ts` to modify:
- Build settings
- CSS processing
- PostCSS plugins
- Development tools

### Tauri Configuration

Edit `src-tauri/tauri.conf.json` to modify:
- App metadata
- Window settings
- Build commands
- Security policies

## 🚀 Deployment

### Web Deployment

The web version can be deployed to any static hosting service:
- Vercel
- Netlify
- GitHub Pages
- AWS S3

### Desktop Distribution

Use Tauri's built-in distribution:
```bash
cd src-tauri
cargo tauri build --release
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes
- `refactor:` Code refactoring
- `test:` Test additions/changes
- `chore:` Build/tooling changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [NuxtJS](https://nuxt.com/) - Vue.js framework
- [TailwindCSS](https://tailwindcss.com/) - Utility-first CSS framework
- [Rust](https://rust-lang.org/) - Systems programming language

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/ibourgeois/nebula-terminal/issues) page
2. Create a new issue with detailed information
3. Include your operating system and version information

---

**Nebula Terminal** - Where modern design meets powerful functionality. ✨
