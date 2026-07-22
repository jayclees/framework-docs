# Routing

---

Sturdy supports dynamic routing and multi route variable segments.

&nbsp;

## Registering routes

---

Your routes will be registered in `src/routes.rs`.

```rust
use sturdy::routing::{Router, Route};

fn register_routes(router: Router) {
    router.get("/", LandingPage);
    router.get("/about", AboutPage);
    
    // Or...
    router.get("/", Page("Landing Title", "Landing Description", "landing.html"));
    router.get("/about", Page("About Title", "About Description", "about.html"));
}
```

&nbsp;

## Route Variables

---

You may register route variables for dynamic route matching.

```rust
router.get("/user/{user}", ShowUser);
```

In your [`Action`](/docs/actions)'s `handle` method:

```rust
// Request: /user/123
let user_id = request.var();

// user_id = "123"
dbg!(user_id);
```

&nbsp;

### Route Variable Constraints

---

You may restrict route variable matches using a [regex](https://docs.rs/regex/latest/regex/) pattern.

```rust
// Use method `getm()` (for get & modify) instead of `get()`
router.getm(
    "/user/{username}",
    ShowUser,
    |route: Route| {
        // path /user/johndoe will hit
        // path /user/123 will not
        route.constrain("username", "[a-zA-Z]+")
    }
);
```

&nbsp;

### Multi Variable Segments

---

You may have multiple tokens per segment.

```rust
router.getm(
    "/post/{author}.{post_id}.{post_slug}",
    ShowPost,
    |route| {
        // matches /post/janedoe.456.how-to-do-thing
        route.constrain("author", "^[a-zA-Z]+")
            .constrain("post_id", "[0-9]+")
            .constrain("post_slug", "[a-zA-Z0-9-]+$")
    });
```

&nbsp;

Next steps:

- [Actions](/docs/actions)
- [Templates](/docs/templates)