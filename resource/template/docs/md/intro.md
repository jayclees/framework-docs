# Why Sturdy Framework?

---

Sturdy aims to reduce the use of macros. Macros can be opaque, and hides much of the underlying functionality. While macros can reduce lines of code, it also obscures how things work under the hood. Sturdy aims to remove these indirections.

Our route endpoints are structs that implement the `Action` trait. This allows you much freedom to design your routing pattern in any way you like.

&nbsp;

```clike
// Method #1:
router.get("/docs/intro", DocsPage { title: "Intro to Sturdy", md: "intro.md" })

// Method #2 (resolve md file using slug in handler):
router.get("/docs/{slug}", DocsPage)

// Etc...
```

&nbsp;

Further reading:

[Install](/docs/install)