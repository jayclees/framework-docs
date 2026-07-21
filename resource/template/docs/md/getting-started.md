# Getting Started

---

To download the project template, `cd` into your preferred projects directory and run the following command:

```shell
git clone https://placeholder my-project
cd my-project
```

&nbsp;

## Running the app

---
There are several ways you can run the environment. We recommend using [Docker](https://docs.docker.com/get-started/docker-overview/) to for service isolation and security.

&nbsp;

### Running with Docker

---

To boot your app using [Docker](https://docs.docker.com/get-started/docker-overview/), run the following command: 

```shell
cd my-project
docker compose up -d --build
```

This will boot up three services:

- The `app` service, which will run your app server and `.rs` file watchers for automatic compilation.
- The `nginx` service, which will proxy requests to your `app` service. This is because Sturdy does not handle serving static assets such as `yourdomain.com/robots.txt` (which will be in `my-project/public` directory).
- The `node` service, which will run `vite` to handle front-end asset bundling, and reloading the client browser when any change is made to front-end or back-end files.

&nbsp;

### Without Docker

---

If you do not wish to use Docker, you may simply run:

```shell
cd my-project/dev-server
cargo run

# In a separate shell
npm install
npm run dev
```

These will boot up the `.rs` file watcher server, and reboot the app server. 

<div class="alert alert-warning">
    Note: Sturdy does not yet support static asset serving. You may set up nginx using the config file in <code>docker/nginx/templates/default.conf.template</code> as a reference.
</div>

&nbsp;

### Vite

---

Sturdy uses [Vite](https://vite.dev/) for front-end asset handling. Vite will watch the files in your `project/resource` directory, compile assets, and send reload requests to the loaded client browser.

Vite is also configured to watch `target/debug/your_app_binary`. When it is modified, Vite will send a reload event to the client browser. This eliminates the need for manual refreshing.

&nbsp;

Further reading:

- [Routing](/docs/routing)
