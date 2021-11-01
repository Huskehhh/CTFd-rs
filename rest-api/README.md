# rest-api

Simple REST API built on [actix-web](https://actix.rs/) for consumption via the ``frontend`` module

#### 🔨 Compilation

1. ```git clone https://github.com/Huskehhh/CTFd-rs && cd CTFd-rs```
2. ```cargo build --release```
3. Done, binary can be found in ``target/release/``

Alternatively... use [Docker](https://www.docker.com/)!

``docker build -f rest-api.Dockerfile -t ctfdrs-rest-api .`` and you're done!

#### 🧰 Setup

1. Make sure you've set up ``ctfdb``
2. Create ``.env`` with the correct values

    ```.env
    DATABASE_URL=mysql://ctfd_bot:password@some.ip.here:3306/ctfd_bot
    BIND_ADDRESS=0.0.0.0:8010
    ALLOWED_ORIGIN=https://api.ctf.husk.pro/
    ```

3. Done!