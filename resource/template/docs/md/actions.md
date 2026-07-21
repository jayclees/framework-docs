# Actions

---

All structs that will be registered as routes must implement the `Action` method. In the `handle` method of the `Action` trait is where your app logic will reside.

Your `handle` method will have the following signature.

```rust
async fn handle(
    &self,
    app: &App,
    request: HttpRequest,
) -> Result<Box<dyn Responsable>, HttpError> {
    Box::new("Hello world!")
}
```

From the `handle` method, a type (wrapped in `Box`) that implements `Responsable` is expected.

These include:

- `&str`
- `String`
- `Vec<usize>`
- `[usize; N]`
- `serde_json::Value`

Support for more coming soon.

&nbsp;

## Template rendering

---

To learn more about templating in Sturdy, visit the following page.

&nbsp;

Further reading:

- [Templates](/docs/templates)