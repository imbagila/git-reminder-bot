# Git Reminder Bot

![version](https://img.shields.io/badge/version-0.1.0-orange)

Lightweight reminder app that will send a Telegram notification if there has been no commit push activity on our GitHub and/or GitLab accounts for today.

> Currently, support is only available for GitLab accounts, and notifications will be sent at 9 AM, 12 PM, 3 PM, 6 PM, and 9 PM every day. Support for GitHub and GitLab self-hosted accounts will be added, as well as support for custom cron (see [TODO](#todo)).

## How to Use

### A. Cargo Run

Simply run `cargo run` with passing pre-defined variables like below :
```bash
GITLAB_USER_ID="123" \
GITLAB_TOKEN="abc123" \
TELEGRAM_BOT_TOKEN="zxc789" \
TELEGRAM_CHAT_ID="-123" \
MESSAGE="You have no push activity today. Go do some commits !" \
TZ="Asia/Jakarta" \
    cargo run
```

### B. Docker Container

First, build the image :
`docker build . -t git-reminder-bot:latest --network=host`

Then run the container with list of environment variables:
```bash
docker run \
    -e GITLAB_USER_ID="123" \
    -e GITLAB_TOKEN="abc123" \
    -e TELEGRAM_BOT_TOKEN="zxc789" \
    -e TELEGRAM_CHAT_ID="-123" \
    -e MESSAGE="You have no push activity today. Go do some commits !" \
    -e TZ="Asia/Jakarta" \
    -d git-reminder-bot:latest
```

## Environment Variables

| Key                | Is Required ? | Default Value | Description            |
| ------------------ | ------------- | ------------- | ---------------------- |
| GITLAB_USER_ID     | **YES**       | -             | Gitlab user ID         |
| GITLAB_TOKEN       | **YES**       | -             | Gitlab token           |
| TELEGRAM_BOT_TOKEN | **YES**       | -             | Telegram bot token     |
| TELEGRAM_CHAT_ID   | **YES**       | -             | Telegram group chat id |
| MESSAGE            | **YES**       | -             | Telegram message       |
| TZ                 | NO            | UTC           | App timezone           |

## TODO

- [ ] Support GitHub account
- [x] Support GitLab account
- [ ] Support GitLab self-host account
- [ ] Custom cron
- [ ] Optional message
- [ ] Use username instead of user id
- [ ] Bot can chat directly to user and manage bot itself
- [ ] Variables should be compiled, not on runtime
