# Why Sturdy Framework?

---

Sturdy aims to reduce the use of macros. While macros can reduce lines of code, they can also obscure how things work under the hood, and can cause errors that are not easy to debug. Sturdy aims to remove these indirections.

Our route endpoints are structs that implement the `Action` trait with a `handle` method. This allows you much freedom to design your routing pattern in any way you like.

`src/routes.js`:
```rust
// Method #1: Shared structs.
router.get("/docs/intro", DocPage { title: "Intro to Sturdy", md: "intro.md" });
router.get("/docs/getting-started", DocPage { title: "Installation", md: "getting-started.md" });

// Method #2 (resolve md file using slug in handler):
router.get("/docs/{slug}", DocPage);

// Method #3: Specific struct for specific route.
router.get("/docs/intro", DocIntroPage);
router.get("/docs/getting-started", DocGettingStartedPage);

// Etc...
```

&nbsp;

Next steps:

- [Install](/docs/install)
