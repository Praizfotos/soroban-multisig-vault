# Contributing to Soroban Multi-Sig Treasury Vault

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/soroban-multisig-vault.git
cd soroban-multisig-vault
```

### 2. Install Dependencies

```bash
make install
```

### 3. Create Branch

```bash
git checkout -b feature/your-feature-name
```

## Development Workflow

### Running Tests

```bash
# All tests
make test

# Contract tests only
cd contracts && cargo test

# Frontend tests
cd frontend && npm test

# Backend tests
cd backend && npm test
```

### Code Style

**Rust**:
```bash
cargo fmt
cargo clippy
```

**TypeScript**:
```bash
npm run lint
npm run type-check
```

### Commit Messages

Follow conventional commits:

```
feat: add new feature
fix: bug fix
docs: documentation update
test: add tests
refactor: code refactoring
chore: maintenance tasks
```

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new features
3. **Run all tests** locally
4. **Create pull request** with clear description
5. **Address review feedback**
6. **Squash commits** if requested

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How was this tested?

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] All tests passing
- [ ] Code follows style guidelines
```

## Areas for Contribution

### Smart Contracts
- Security improvements
- Gas optimization
- New proposal types
- Additional features

### Frontend
- UI/UX improvements
- New components
- Accessibility enhancements
- Performance optimization

### Backend
- API improvements
- Event indexing optimization
- Additional endpoints
- Monitoring tools

### Documentation
- Tutorials
- Examples
- Translations
- API documentation

### Testing
- Additional test cases
- Security tests
- Integration tests
- E2E tests

## Questions?

Open an issue or join our Discord for help!
