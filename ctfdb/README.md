# ctfdb

A database library module built on [diesel](https://diesel.rs/) to abstract away database interactions and API calls.

#### 🧰 Setup

1. Install [diesel_cli](https://diesel.rs) -> ``cargo install diesel_cli --no-default-features --features mysql``
2. Create ``.env`` with the correct values

    ```.env
    DATABASE_URL=mysql://ctfd_bot:password@some.ip.here:3306/ctfd_bot
    ```

3. Run diesel setup -> ``diesel setup``