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
There are several ways you can run the environment. We recommend using [Docker](https://docs.docker.com/get-started/docker-overview/) to for service isolation and security. If you do not wish to run your environment using Docker, you may skip to the "Without Docker" section below.

&nbsp;

## Running with Docker

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

Next you will need to find the IP address of the Nginx container. This is usually one of:

- 172.19.0.2
- 172.19.0.3
- 172.19.0.4

To find the exact IP address, run the following:

```shell
docker compose ps
```

You will see output similar to:

```
NAME                 IMAGE              COMMAND                  SERVICE   CREATED          STATUS          PORTS
my-project-app-1     my-project-app     "/bin/bash -c 'cargo…"   app       10 seconds ago   up 9 seconds   0.0.0.0:3000->3000/tcp, [::]:3000->3000/tcp
my-project-nginx-1   my-project-nginx   "/docker-entrypoint.…"   nginx     10 seconds ago   up 9 seconds   80/tcp
my-project-node-1    my-project-node    "docker-entrypoint.s…"   node      10 seconds ago   up 9 seconds
```

We will use `my-project-nginx-1` to run `docker inspect` on. This will give us the local IP address your website will be running on.

```shell
# Get IP address of Nginx container
docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' \
    my-project-nginx-1
# 172.19.0.x
```

The resulting IP address is where your website will be accessible.

&nbsp;

&nbsp;

## Without Docker

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

## Vite

---

Sturdy uses [Vite](https://vite.dev/) for front-end asset handling. Vite will watch the files in your `project/resource` directory, compile assets, and send reload requests to the loaded client browser.

Vite is also configured to watch `target/debug/your_app_binary`. When it is modified, Vite will send a reload event to the client browser. This eliminates the need for manual refreshing.

&nbsp;

Next steps:

- [Routing](/docs/routing)
