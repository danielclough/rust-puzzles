# Rust Quizzes - Technical Design Document

Author: Daniel Clough

## Introduction

### Rationale

> Companies building large teams of Rust users report that the typical onboarding time for a Rust engineer is around 3-6 months.
> -- [rust-lang.org](https://blog.rust-lang.org/inside-rust/2022/04/04/lang-roadmap-2024.html)

Learning Rust is a long, complex process. While the community is growing quickly, there are still relativly few resources for new Rustaceans.

Beginner, intermediate, and advanced learners all need regular feedback in order to improve.
Rust Quizzes can offer feedback with regards benchmarking, best practices, all within the comfort of your favorite text editor or IDE.

This project was inspired by Hacker Rank, but offers...
 - a simple Text User Interface (TUI) for selecting quizzes
 - open source code
 - live feedback
 - code benchmarking

### Background

Describe any historical context that would be needed to understand the document, including legacy considerations.

### Terminology

 - TUI == Text User Interface
 - Rustacean == Someone who codes in Rust

### Non-Goals

 - This is not a platform for competition, or head hunting.
 - There will not be any user accounts.
 - Benchmarks are anonymous.


### Proposed Design

#### Layout/Interaction

##### Home (wide layout)
Page for displaying everything but lists.
Including:
 - welcome
 - help
 - log trial in progress

##### Start (no layout)
Start Time Trial

##### List (2 col layout)

List all Quizzes:
| Name | Lvl. | Desc. | Example | Constraints | Input | Output |
| -- | -- | -- | -- | -- | -- | -- |
| Name that corrosponds to Enum | Not a very accurate ranking | Description of Quiz | What you need to get from input to output | Constraints on input data | Input Example | Output Example |


##### Results (2 col layout)

List all Results:
| Attempt | Results |
| -- | -- |
| List of user attempts | **Name**, **Lvl.**, **Speed**, **Benchmarks** |

*Submenu:*
| Name | Lvl. | Speed | **Benchmarks** |
| -- | -- | -- | -- |
| Name that corrosponds to Enum | Level | How long the time trial took. | Submenu |

*Benchmarks Submenu TBD*

##### Quit (no layout)
Cleanup and exit.

### System Architecture

#### Quizzes

#### TUI

#### API

### Data Model

Describe how the data is stored. This could include a description of the database schema.

### Interface/API Definitions

Describe how the various components talk to each other. For example, if there are REST endpoints, describe the endpoint URL and the format of the data and parameters used.

### Business Logic

If the design requires any non-trivial algorithms or logic, describe them.

### Impact

Describe the potential impacts of the design on overall performance, security, and other aspects of the system.

### Risks

If there are any risks or unknowns, list them here. Also if there is additional research to be done, mention that as well.

### Alternatives

#### Coding Challanges
 - [Project Euler](https://projecteuler.net)
 - [Advent of Code](https://adventofcode.com)
 - [Hacker Rank](https://https://www.hackerrank.com/challenges)

#### Interactive Rust Education
 - [Rustlings](https://github.com/rust-lang/rustlings/)
