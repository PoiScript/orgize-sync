# orgize-sync

Sync your Org with your favorite applications.

> This project is still in *alpha stage*. Don't forget to backup your
> orgmode files before trying!

## Installation

```
$ cargo install orgize-sync
```

## Subcommand

### `init`

Initializes a new configuration file

```
USAGE:
    orgize-sync init [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Increases verbosity
```

### `conf`

Prints your configuration file

```
USAGE:
    orgize-sync conf [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Increases verbosity

OPTIONS:
    -c, --conf <conf-path>    Path to configuration file
```

### `sync`

Synchronizes org files

```
USAGE:
    orgize-sync sync [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                    Prints help information
        --skip-google-calendar    Skips Google Calendar synchronization
        --skip-toggl              Skips Toggl synchronization
    -V, --version                 Prints version information
    -v, --verbose                 Increases verbosity

OPTIONS:
    -c, --conf <conf-path>    Path to configuration file
```

## Configuration

+ [General](#general)
  + [Global](#global)
  + [Pre-file](#pre-file)
+ [Google Calendar](#google-calendar)
  + [Global](#global-1)
  + [Pre-file](#pre-file-1)
+ [Toggl](#toggl)
  + [Global](#global-2)
  + [Pre-file](#pre-file-2)

### General

#### Global

```javascript
{
    // Path to dotenv file.
    // The default is `${UserCacheDir}/orgize-sync/.env`.
    "env_path": "./.env",
    // Number of days to filter headline before today.
    // The default is 7.
    "up_days": 1,
    // Number of days to filter headline after today.
    // The default is 7.
    "down_days": 1
}
```

#### Pre-file

```javascript
{
    "files": [{
        // Specifies the name for this orgmode file. Optional.
        "name": "note",
        // Specifies the path to orgmode file. Required.
        "path": "./notes.org"
    }]
}
```

### Google Calendar

#### Global

```javascript
{
    "google-calendar": {
        // Google OAuth client id. Required.
        // Sepcifying here or by setting the `GOOGLE_CLIENT_ID` environment variable.
        "client_id": "xxx",
        // Google OAuth client secret. Required.
        // Sepcifying here or by setting the `GOOGLE_CLIENT_SECRET` environment variable.
        "client_secret": "xxx",
        // Redirect url after authorizing.
        // The default is `http://localhost`
        "redirect_uri": "",
        // Path to store the access token and refresh token.
        // The default is `${UserCacheDir}/orgize-sync`.
        "token_dir": "",
        // The default is `google-token.json`.
        "token_filename": ""
    }
}
```

#### Pre-file

```javascript
{
    "files": [{
        "google-calendar": {
            // Which calendar to sync. Required.
            "calendar": "",
            // Whether to append new calendar event to the org mode.
            // The default is true.
            "append_new": false,
            // Where to append new calendar event.
            // The default is `Sync`.
            "append_headline": "",
            // Which property to store event id.
            // The default is "EVENT_ID`.
            "property": ""
        }
    }]
}
```

### Toggl

#### Global

```javascript
{}
```

#### Pre-file

```javascript
{}
```

## License

MIT
