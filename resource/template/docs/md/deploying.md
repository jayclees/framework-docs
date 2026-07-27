# Launching your website

---

The following are high level steps to deploying your website on the web.

- Create a VPS (Linux, Ubuntu tested)
- Install dependencies
  - Rust
  - Nginx
  - Certbot (if you want Let's Encrypt/ssl)
- Clone your project
- Run `cargo build --release`
- Create `/etc/nginx/sites-enabled/mywebsite.tld.conf`

`mywebsite.tld.conf`:

```
server {
    server_name  mywebsite.tld;

    #access_log  /var/log/nginx/access.log  main;

    location / {
        # Important that you point to my-project/public so your sensitive files such
        # as .env and your project source files are not publicly accessible.
        root /home/sammy/my-project/public;
        try_files $uri $uri/ @app;
        gzip_static on;
    }

    location = / {
        proxy_pass http://127.0.0.1:3000$uri?$query_string;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location @app {
        proxy_pass http://127.0.0.1:3000$uri?$query_string;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

- Run `service nginx reload`
- If you want SSL, you may use [`certbot`](https://certbot.eff.org/) (among other options)
- Run `cargo run --release`
- Update your DNS entries to point to your VPS's IP address
- Visit mywebsite.tld (replace with your domain name)
