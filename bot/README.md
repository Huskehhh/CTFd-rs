# bot

A [CTFd](https://github.com/CTFd/CTFd) aware Discord bot built on [serenity-rs](https://github.com/serenity-rs/serenity) with functionality that aims to help collaboration of a team throughout a CTF.

Included with the bot is a polling task, that will continuously check for new updates via [CTFd](https://github.com/CTFd/CTFd) API

#### 🔨 Compilation

1. ```git clone https://github.com/Huskehhh/CTFd-rs && cd CTFd-rs```
2. ```cargo build --release```
3. Done, binary can be found in ``target/release/``

Alternatively... use [Docker](https://www.docker.com/)!

``docker build -f bot.Dockerfile -t ctfdrs-bot .`` and you're done!

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
    GUILD_ID=000000000000000000
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
| !htb link <*htb id*> <*discord id*>                                         | Links the provided HTB ID with a Discord ID                                                    | Organiser  |
| !ctf active                                                                 | Lists all active CTFs                                                                          | CTFer      |
| !ctf list "*ctf name*" OR !ctf list                                         | Lists all challenges on given CTF OR for CTF linked to current channel                         | CTFer      |
| !ctf [working/w] "*challenge*"                                              | Marks you as working on the given challenge                                                    | CTFer      |
| !ctf [giveup/g] "*challenge*"                                               | Removes you from working on the given challenge                                                | CTFer      |
| !ctf [search] "*challenge*"                                                 | Searches for the given challenge and returns the status of it                                  | CTFer      |
| !ctf stats                                                                  | Displays the current stats for all active CTFs                                                 | CTFer      |
| !htb [working/w] "*challenge*"                                              | Marks you as working on the given challenge                                                    | CTFer     |
| !htb [giveup/g] "*challenge*"                                               | Removes you from working on the given challenge                                                | CTFer     |
| !htb [search] "*challenge*"                                                 | Searches for the given challenge and returns the status of it                                  | CTFer     |
