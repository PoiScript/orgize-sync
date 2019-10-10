# orgize-sync

Sync your Org with your favorite applications.

**Note**: This project is still in *alpha stage*. Don't forget to backup
your orgmode files before trying!

## Commands

### `Init`

// TODO

### `Sync`

// TODO

### `Conf`

// TODO

## Configuration

### General

#### Global

```javascript
{
    // path to dotenv file
    // default is "${UserCacheDir}/orgize-sync/.env"
    "env_path": "./.env",
    // number of days to filter headline before today
    // default is 7
    "up_days": 1,
    // number of days to filter headline after today
    // default is 7
    "down_days": 1
}
```

#### Pre-file

```javascript
{
    "files": [{
        // specify a name for this file, optional
        "name": "note",
        // path to this orgmode file, required
        "path": "./notes.org"
    }]
}
```

### Google Calendar

#### Global

```javascript
{
    "google-calendar": {
        // google oauth client id, required
        // specifying here or by setting the GOOGLE_CLIENT_ID environment variable
        "client_id": "xxx",
        // google oauth client_secret
        // sepcifying here or by setting the GOOGLE_CLIENT_SECRET environment variable
        "client_secret": "xxx",
        // redirect url after authorizing
        // default is "http://localhost"
        "redirect_uri": "",
        // where to store the access token and refresh token
        // default is "${UserCacheDir}/orgize-sync"
        "token_dir": "",
        // default is "google-token.json"
        "token_filename": ""
    }
}
```

#### Pre-file

```javascript
{
    "files": [{
        "google-calendar": {
            // which calendar to sync, required
            "calendar": "",
            // whether to append new calendar event to the org mode
            // default is true
            "append_new": false,
            // where to append new calendar event
            // default is "Sync"
            "append_headline": "",
            // which property to store event id
            // default is "EVENT_ID"
            "property": ""
        }
    }]
}
```

## License

MIT
