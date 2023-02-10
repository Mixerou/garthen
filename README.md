<h1 align="center">
    <span>GARTHEN</span>
    <br>
    <span>🌱 Greenhouse tools 🌱</span>
</h1>

## Content

- [Commit Lint](#commit-lint)
  - [Explanation](#explanation)
  - [Installation](#installation)
  - [Usage](#usage)
- [Versioning](#versioning)
  - [Explanation](#explanation-1)

## Commit Lint

To keep the repository clean
and to make it easy for new developers to enter the project,
we use `commitlint`.

### Explanation

All commits in this repo look like this

```text
<Header>
<BLANK LINE>
<Body>
<BLANK LINE>
<BLANK LINE>
<Footer>
```

Header

```text
type(scope)!: subject
```

#### Type

Type is a super short info about this commit.
Use it to communicate your intent to the project developers.

- `fix` - patches a bug in the codebase
- `feat` - introduces a new feature to the codebase
- `refactor` - a code change that neither fixes a bug nor adds a feature
- `perf` - a code change that improves performance
- `build` - changes that affect the build system or external dependencies
- `chore` - chores that are regularly done for maintenance purposes
- `revert` - revert commits

#### Scope

- `*` - it selects all available scopes
- `repo` - changes that not effect project modules (example: ci, readme, etc.)
- `web-cl` - changes that effects `Web Client` module
- `gl-api` - changes that effects `Global API` module

#### Exclamation mark

If a commit has a footer `BREAKING CHANGE:`,
or appends a `!` after the type/scope,
it introduces a breaking codebase changes.

#### Subject

Summary in present tense. Not capitalized. No period at the end.

#### Revert Commits

If the commit reverts a previous commit, it should have `revert` type,
followed by the header of the reverted commit.

The content of the commit message body should contain:

- information about the SHA of the commit being reverted in the following format: `This reverts commit <SHA>`
- a clear description of the reason for reverting the commit message.

### Installation

Use the following commands to start using it.

> You must have [Node JS](https://nodejs.org/) installed

```bash
$ npm install -g @commitlint/cli

$ npx husky install
```

### Usage

```bash
$ git commit -m "refactor(repo): fix typo in README file"
```

## Versioning

All modules in this repository use [semantic versioning](https://semver.org/) and the same version.

### Explanation

Given a version number **MAJOR.MINOR.PATCH**, increment the:

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner
- **PATCH** version when you make backwards compatible bug fixes

Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format.
