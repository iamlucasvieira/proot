# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3]

### Added

- Default number of PRs to display in the tree and option to change it

## [0.2.2]

### Added

- Functionality to filter PRs based on their dependencies
- Allow to filter custom PRs

### Changed

- Updated the README with the new functionality
- Updated main CLI logic, adding subcommands for check and filter
- Updated the PR_NUMBER type to be a integer, allowing CLAP validation

### Fixed

- Fixed display of child PRs, making sure all lines are displayed correctly

## [0.2.1]

### Added

- Functionality to open a PR in the browser from the CLI

### Changed

- Updated the README with the new functionality

## [0.2.0]

### Added

- Functionality to display the tree of PRs
- Functionality to display version of CLI
