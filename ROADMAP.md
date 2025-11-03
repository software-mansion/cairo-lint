# Cairo Lint Roadmap

A list of main features and tasks that are planned to be added for Cairo lint. 

The list is sorted from top to bottom by the priority that each task has. The higher the priority, the higher the place of the task in the list.

For more detailed info, look into the [Board](https://github.com/orgs/software-mansion/projects/33/views/7) into the `Backlog Lint` and `Todo` sections.

## Q4 2025 and Q1 2026

### 1. Fixing any existing bugs

We still have some bugs to be repaired. Each of them relates to certain lints, so the effort required to fix them should be rather small.

### 2. Upstreaming shared logic with Cairo Language Server

A lot of the critical code parts are just blatantly copied from Language Server. We want to create a separate crate that will contain all the shared logic between those two. This way we get a single source of truth, and it will be much easier to maintain.

### 3. Add various new lints and fixers

We want to add new lint rules (and corresponding fixer if possible), such as `collapsible_match`, `disallowed_methods`, `inefficient_unwrap_or`, `impossible_comparison` and much more. The list of lints waiting to be implemented is very long, but those mentioned should be the priority during this period.

### 4. De Morgan's resolving algorithm

Most of our fixers create or modify existing bool expressions. Unfortunately right now we are using the simplest way to do that, and because of that, fixed code samples look much more complicated than they need to be. With this algorithm for the Cairo expressions, we should be able to simplify all the expressions that we are interested in.
