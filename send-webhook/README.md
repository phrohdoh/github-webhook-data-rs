# `send-webhook`

This project produces an executable that will [`POST`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST) [`JSON`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON) data that matches the [`PullRequestEvent`](https://developer.github.com/v3/activity/events/types/#pullrequestevent) data type of the GitHub webhook API to a given address.

### Running

```
send-webhook http://127.0.0.1:3000/endpoint
```

or via [`cargo`](http://doc.crates.io):

```
cargo run http://127.0.0.1:3000/endpoint
```