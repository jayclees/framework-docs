# Templates
---
Sturdy uses the battle-tested <code>[minijinja](https://docs.rs/minijinja/latest/minijinja/)</code> library for templating.

Your templates reside in `resource/template`. `minijinja` is language agnostic, so you may use `.html`, `.md` or any other type of file you'd like.

&nbsp;

## Rendering Templates
---
```rust
async fn handle(
    &self,
    app: &App,
    request: HttpRequest,
) -> Result<Box<dyn Responsable>, HttpError> {
    // project-root/resource/template/user.html
    let result = app.template("user.html", context! { name: "John Doe" });

    match result {
        Ok(rendered) => Ok(Box::new(rendered)),
        Err(error) => Err(HttpError::new(500, error.to_string())),
    }
}
```

Read more: [`minijinja::context`](https://docs.rs/minijinja/latest/minijinja/macro.context.html).

&nbsp;

## Template Interpolation
---
You may replace tokens in your templates for variable values.

```jinja2
<p>Name: {{ user.name }}</p>
```

&nbsp;

## Comments
---
```jinja2
<p>This will be rendered.</p>
{# This will not be rendered. #}
```

&nbsp;

## Control Flow
---
Control if content is displayed, loop over iterators.

&nbsp;

### If / elif / else
---
```jinja2
<p>
    Role:
    {% if user.is_admin %}
        Admin
    {% elif user.is_mod %}
        Moderator
    {% else %}
        Member
    {% endif %}
</p>
```

&nbsp;

### Loops
---
Assuming type: `Vec<String>`:
```jinja2
<ul>
    {% for todo in todos %}
        <li>{{ todo }}</li>
    {% endfor %}
</ul>
```
Assuming type: `HashMap<String, User>`:
```jinja2
<ul>
    {% for slug, post in posts|items %}
        <li>{{ post.title }}</li>
    {% endfor %}
</ul>
<ul>
    {% for id, user in posts|dictsort %}
        <li>{{ user.title }}</li>
    {% endfor %}
</ul>
```
Note the `|items` modifier after `posts`.

&nbsp;

## Template inheritance
---
Extend layouts for easy repetition and reuse.

Parent template `base.html`:
```jinja2
<!doctype html>
<html>
<head>
    {% block head %}
        <title>My Awesome Website</title>
    {% endblock %}
</head>
<body>
{% block content %}{% endblock %}
</body>
</html>
```

Child template `my-page.html`:
```jinja2
{% extends "base.html" %}
{% block head %}
    <title>My Awesome Page</title>
{% endblock %}
{% block content %}
    <p>Hello world!</p>
{% endblock %}
```

&nbsp;

Links:

- [`minijinja` docs](https://docs.rs/minijinja/latest/minijinja/)
- [`minijinja` template syntax](https://docs.rs/minijinja/latest/minijinja/syntax/index.html#tags)
- [Jinja templating reference](https://jinja.palletsprojects.com/en/stable/templates/)

Next steps:

- ...