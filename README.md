# twikit (Rust + PyO3)

This repository provides a minimal Rust implementation of core twikit components and Python bindings via PyO3 / maturin.

Build & install into your active Python environment (recommended):

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop -i python3
python3 examples/python_example.py
```

Or build a wheel:

```bash
maturin build --release
```

A Simple Twitter API Scraper

You can use functions such as posting or searching for tweets without an API key using this library.

- [Documentation (English)](https://twikit.readthedocs.io/en/latest/twikit.html)


🔵 [Discord](https://discord.gg/nCrByrr8cX)

> [!NOTE]
> Released twikit_grok an extension for using Grok AI with Twikit.  
> For more details, visit: https://github.com/d60/twikit_grok.




## Features

### No API Key Required

This library uses scraping and does not require an API key.

### Free

This library is free to use.


## Functionality

By using Twikit, you can access functionalities such as the following:

-  Create tweets

-  Search tweets

-  Retrieve trending topics

- etc...



## Installing

```bash

pip install twikit

```



## Quick Example

**Define a client and log in to the account.**

```python
import asyncio
from twikit import Client

USERNAME = 'example_user'
EMAIL = 'email@example.com'
PASSWORD = 'password0000'

# Initialize client
client = Client('en-US')

async def main():
    await client.login(
        auth_info_1=USERNAME,
        auth_info_2=EMAIL,
        password=PASSWORD,
        cookies_file='cookies.json'
    )

asyncio.run(main())
```

**Create a tweet with media attached.**

```python
# Upload media files and obtain media_ids
media_ids = [
    await client.upload_media('media1.jpg'),
    await client.upload_media('media2.jpg')
]

# Create a tweet with the provided text and attached media
await client.create_tweet(
    text='Example Tweet',
    media_ids=media_ids
)

```

**Search the latest tweets based on a keyword**
```python
tweets = await client.search_tweet('python', 'Latest')

for tweet in tweets:
    print(
        tweet.user.name,
        tweet.text,
        tweet.created_at
    )
```

**Retrieve user tweets**
```python
tweets = await client.get_user_tweets('123456', 'Tweets')

for tweet in tweets:
    print(tweet.text)
```

**Send a dm**
```python
await client.send_dm('123456789', 'Hello')
```

**Get trends**
# twikit — Rust library + PyO3 bindings

This repository contains a Rust implementation of core twikit components and Python bindings built with PyO3 and maturin.

This README is focused on Rust developers and on how to build and test the Python extension from this repository.

**What this repo provides**
- A Rust library exposing a Python module named `twikit` (see `src/lib.rs`).
- Small Rust modules with PyO3 bindings: `Bookmark`, `Community`, `Geo`, `User`, `Tweet`, `Client`, `Media`, `Stream`, `NotificationManager`.

**Goals**
- Provide fast Rust implementations for core functionality.
- Export a lightweight, importable Python module via PyO3/maturin.

**Quick Rust developer commands**
- Check the code compiles: `cargo check`
- Build (release): `cargo build --release`
- Run unit tests: `cargo test`
- Format code: `cargo fmt`

**Build Python extension (using your active Python — conda or system)**

Use `maturin` to build and install the extension into your active Python environment.

Recommended: use your current conda env (no venv) or activate the Python you want to use.

Example (using conda env):

```bash
# from repo root
python3 -m pip install --upgrade maturin
# build & install in-place into the active Python
maturin develop
```

If you prefer a virtualenv instead of conda:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
```

After installation you can run the example script:

```bash
python3 examples/python_example.py
```

**Notes**
- If `maturin develop` fails because both `VIRTUAL_ENV` and `CONDA_PREFIX` are set, run the command from the desired environment only (unset one of them in your shell for the build process).
- The Python module name is `twikit` and is defined in `src/lib.rs`.

**API (exported types)**
- `Bookmark` — bookmark manager
- `Community` — community manager
- `Geo` — geo utilities (haversine distance)
- `User` — user store
- `Tweet` — tweet structure
- `Client` — simple client with `post_tweet`/`get_tweet`
- `Media` — media upload helper
- `Stream` — streaming helper that calls Python callbacks
- `NotificationManager` — mock notification sender

**Testing the Python API**
After `maturin develop` you can import the module in Python:

```python
import twikit

b = twikit.Bookmark()
b.add('https://example.com')
print(b.list())

client = twikit.Client()
t = client.post_tweet('hello from rust', 'alice')
print(t.to_dict())
```

**Contributing / next steps**
- Expand Rust implementations for modules you need (I can help port more of the original Python API).
- Add integration tests and CI to build wheels automatically.

If you want me to run the build in your active conda environment now, tell me and I'll run `maturin develop` and execute the example.
