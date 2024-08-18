A small REST-endpoint generating passwords.

Based on Rust and the Rocket web-framework and the password crate passwords.

Build and run the docker-image

```
docker build -t rocket_tutorial:dev .
docker run -p 8000:8000 rocket_tutorial:dev
```

Calling `/pwd` will return five passwords. Calling `/pwd/x` will return up to 31 passwords.

Rocket framework: https://rocket.rs/

Password-crate: https://docs.rs/passwords/latest/passwords/index.html
