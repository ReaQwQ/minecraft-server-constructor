# Contributing to MSB

First off, thank you for considering contributing to MSB! It's people like you that make MSB such a great tool for the Minecraft community.

## 📜 Code of Conduct

By participating in this project, you agree to abide by our standards of professionalism and respect.

## 🛠 Development Guidelines

### Rust (Backend)
- **Zero-Cost Abstractions**: Always prefer borrowing over cloning. Unnecessary `.clone()` calls will be flagged in review.
- **Safety First**: `unsafe` code is strictly prohibited unless absolutely necessary for performance and accompanied by a detailed justification.
- **Error Handling**: Use `thiserror` for library errors (`msb-core`) and `anyhow` for application-level logic.
- **Documentation**: All public functions MUST have a doc block in the following format:
  ```rust
  /**
   * 説明: [Purpose]
   * @param [name] [description]
   * @requires [dependencies]
   * @return [meaning]
   */
  ```

### React (Frontend)
- **Functional Components**: Use functional components with hooks.
- **Styling**: Use Tailwind CSS and the `cn()` utility for conditional classes.
- **Glassmorphism**: When adding new UI elements, ensure they conform to the "Liquid Glass" theme using the `.glass` class.
- **i18n**: Never hardcode strings. Always add keys to `app/src/lang/en.json` and other relevant language files.

## 🚀 Pull Request Process

1.  **Fork the repository** and create your branch from `main`.
2.  **Ensure the build passes**:
    - Run `cargo check` and `cargo clippy`.
    - Run `npm run build`.
3.  **Update documentation**: If you've added new features, ensure they are documented in both the code and the README if necessary.
4.  **Submit your PR**: Provide a clear description of the changes and the problem they solve.

## 💎 Design Philosophy

MSB aims for a "Native OS" feel. Animations should be fluid (using `framer-motion`) and the UI should be non-intrusive.

---

Thank you for your contributions!
