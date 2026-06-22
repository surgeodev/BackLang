# Contributing to BackLang Debug Extension

Thank you for your interest in contributing to the BackLang VS Code Debug Extension! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and adhere to our Code of Conduct in all interactions.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem** in as many details as possible
- **Provide specific examples** to demonstrate the steps
- **Describe the behavior you observed** after following the steps
- **Explain which behavior you expected** to see instead and why
- **Include screenshots and animated GIFs** if possible

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description** of the suggested enhancement
- **Provide specific examples** to demonstrate the steps
- **Describe the current behavior** and **the expected behavior**
- **Explain why this enhancement would be useful**

### Pull Requests

- Fill in the required template
- Follow the TypeScript/JavaScript styleguide
- Include appropriate test cases
- End all files with a newline
- Avoid platform-specific code

## Development Setup

### Prerequisites

- Node.js 16+
- npm or yarn
- VS Code 1.70+
- git

### Getting Started

1. **Fork the repository**

   ```bash
   git clone https://github.com/YOUR-USERNAME/vscode-backlang-debug.git
   cd vscode-backlang-debug
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Compile TypeScript**

   ```bash
   npm run compile
   ```

4. **Watch for changes**

   ```bash
   npm run watch
   ```

5. **Test the extension**
   - Press `F5` in VS Code to launch a debug instance
   - Open a `.bl` file to test

### Project Structure

```
vscode-extension/
├── src/
│   ├── extension.ts          # Main extension entry point
│   └── debugAdapter.ts       # Debug adapter implementation
├── syntaxes/
│   └── backlang.tmLanguage.json  # Syntax highlighting rules
├── examples/                 # Example BackLang programs
├── .vscode/                  # VS Code configuration
├── package.json              # Extension manifest
├── tsconfig.json             # TypeScript configuration
└── README.md                 # Documentation
```

## Styleguide

### TypeScript/JavaScript

- Use PascalCase for classes
- Use camelCase for functions, variables, and properties
- Use UPPER_SNAKE_CASE for constants
- Use meaningful variable names
- Add JSDoc comments for public APIs
- Keep functions small and focused (max 50 lines ideally)

Example:

```typescript
/**
 * Creates a debug session for a BackLang file
 * @param filePath - Path to the .bl file
 * @returns Debug session or null if creation fails
 */
function createDebugSession(filePath: string): DebugSession | null {
  // Implementation
}
```

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Example:

```
Add support for conditional breakpoints

- Implement conditional breakpoint parsing
- Add condition evaluation in debug adapter
- Update UI to show condition in breakpoint editor

Fixes #123
```

### Documentation

- Use clear and concise language
- Include code examples where appropriate
- Keep README.md up to date
- Document API changes in CHANGELOG.md

## Testing

### Running Tests

```bash
npm test
```

### Writing Tests

- Place tests in the `test/` directory
- Use descriptive test names
- Test both happy path and error cases
- Maintain test coverage above 70%

Example test:

```typescript
describe('BackLang Extension', () => {
  it('should create a debug session for valid .bl files', () => {
    // Test implementation
  });
});
```

## Additional Notes

### Issue and Pull Request Labels

- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Improvements or additions to documentation
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention is needed
- `question` - Further information is requested
- `wontfix` - This will not be worked on

### Community

- Join our Discord server for discussion
- Read existing issues and discussions
- Help answer questions from other users
- Share your experience with the extension

## Recognition

Contributors will be recognized in:

- The CHANGELOG.md file
- The README.md file (contributors section)
- GitHub's contributor list

## Questions?

Don't hesitate to ask questions! You can:

- Open a discussion on GitHub
- Create an issue labeled "question"
- Reach out to maintainers directly

Thank you for contributing to BackLang! 🎉
