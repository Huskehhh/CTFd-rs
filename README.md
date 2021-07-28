# Welcome to CTFd-rs [![GitHub Actions CI](https://github.com/Huskehhh/CTFd-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Huskehhh/CTFd-rs/actions/workflows/ci.yml)

A project that aims to provide utility tooling for teams competing in [CTFd](https://github.com/CTFd/CTFd) hosted CTFs.

## Modules, and setup

Prerequisites

- [Rust](https://www.rust-lang.org/)
- [git](https://git-scm.com/)
- [MySQL](https://www.mysql.com/) / [Maria](https://mariadb.org/) database
- ``default-libmysqlclient-dev`` or equivalent
- (Optional) [Docker](https://www.docker.com/)

### ctfdb

#### Required module

A database library module built on [diesel](https://diesel.rs/) to abstract away database interactions and API calls.

#### 🧰 Setup

1. Install [diesel_cli](https://diesel.rs) -> ``cargo install diesel_cli --no-default-features --features mysql``
2. Create ``.env`` with the correct values

    ```.env
    DATABASE_URL=mysql://ctfd_bot:password@some.ip.here:3306/ctfd_bot
    ```

3. Run diesel setup -> ``diesel setup``

### bot

#### Required module

A [CTFd](https://github.com/CTFd/CTFd) aware Discord bot built on [serenity-rs](https://github.com/serenity-rs/serenity) with functionality that aims to help collaboration of a team throughout a CTF.

Included with the bot is a polling task, that will continuously check for new updates via [CTFd](https://github.com/CTFd/CTFd) API

#### 🔨 Compilation

1. ```git clone https://github.com/Huskehhh/CTFd-rs && cd CTFd-rs/bot```
2. ```cargo build --release```
3. Done, binary can be found in ``target/release/``

Alternatively... use [Docker](https://www.docker.com/)!

``docker build -f Dockerfile.bot -t bot .`` and you're done!

#### 🧰 Setup

1. Make sure you've set up ``ctfdb``
2. Create ``.env`` with the correct values

    ```.env
    DATABASE_URL=mysql://ctfd_bot:password@some.ip.here:3306/ctfd_bot
    DISCORD_TOKEN=<token goes here>
    OWNER_ID=276519212100000000
    HTB_TEAM_ID=0
    HTB_EMAIL=some@email.here
    HTB_PASSWORD=supersecretpassword123
    HTB_CHANNEL_ID=860092136775200000
    ```

   See [here](https://discord.com/developers/docs/topics/oauth2#bots) for more information.
   TL;DR generate bot token from [here](https://discord.com/developers/applications)

3. Done!

#### 💻 Commands

Available prefixes are ``!`` ``.`` ``~``

| Command                                                                     | Description                                                                                    | Permission |
| --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ---------- |
| !ctf start "*ctf name*" *https://url.to.ctf* *api-key* *discord-channel-id* | Starts a new CTF with given name, uses API key as auth. Posts updates to given discord channel | Organiser  |
| !ctf end "*ctf name*"                                                       | Ends CTF with given name                                                                       | Organiser  |
| !ctf active                                                                 | Lists all active CTFs                                                                          | CTFer      |
| !ctf list "*ctf name*" OR !ctf list                                         | Lists all challenges on given CTF OR for CTF linked to current channel                         | CTFer      |
| !ctf [working/w] "*challenge*"                                              | Marks you as working on the given challenge                                                    | CTFer      |
| !ctf [giveup/g] "*challenge*"                                               | Removes you from working on the given challenge                                                | CTFer      |
| !ctf [search] "*challenge*"                                                 | Searches for the given challenge and returns the status of it                                  | CTFer      |
| !ctf stats                                                                  | Displays the current stats for all active CTFs                                                 | CTFer      |
| !htb [working/w] "*challenge*"                                              | Marks you as working on the given challenge                                                    | CTFer     |
| !htb [giveup/g] "*challenge*"                                               | Removes you from working on the given challenge                                                | CTFer     |
| !htb [search] "*challenge*"                                                 | Searches for the given challenge and returns the status of it                                  | CTFer     |

### rest-api

#### Optional module

Simple REST API built on [actix-web](https://actix.rs/) for consumption via the ``frontend`` module

#### 🔨 Compilation

1.```git clone https://github.com/Huskehhh/rest-api && cd CTFd-rs/rest-api```
2. ```cargo build --release```
3. Done, binary can be found in ``target/release/``

Alternatively... use [Docker](https://www.docker.com/)!

``docker build -f Dockerfile.rest-api -t rest-api .`` and you're done!

#### 🧰 Setup

1. Make sure you've set up ``ctfdb``
2. Create ``.env`` with the correct values

    ```.env
    DATABASE_URL=mysql://ctfd_bot:password@some.ip.here:3306/ctfd_bot
    BIND_ADDRESS=0.0.0.0:8010
    ALLOWED_ORIGIN=https://api.ctf.husk.pro/
    ```

3. Done!

### frontend

#### Optional module

Frontend to display the CTF challenges and status, built with [React](https://reactjs.org/)

#### 🔨 Compilation

1. ``npm run build``

#### 🧰 Setup

A single environment variable is required for the frontend, and that is ``API_URL`` which should look like ``https://api.ctf.husk.pro``

## ⚠️ Issues/Enhancements

[Issue Tracker](https://github.com/Huskehhh/CTFd-rs/issues)
