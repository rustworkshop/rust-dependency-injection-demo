# Rust Dependency Injection Demo

by [Tim Abell](https://timwise.co.uk/)

This repo is to try out the various ways to swap out dependencies for fakes/mocks/stubs/doubles etc. for testing.

GPT came up with some ideas here: <https://gist.github.com/timabell/8813d851399908987396c1725aa8b6d6>

Running main will show how many stars there are on a repo by way of a http api call to github. But for tests that will be replaced with a test harness so the tests can run fast, offline, and will not "flap" based on availability of github.

You can see the equivalent by running:

```bash
curl --silent https://api.github.com/repos/timabell/gitopolis | jq '.stargazers_count'
```

Which pulls the json data from <https://api.github.com/repos/timabell/gitopolis> and uses [jq](https://stedolan.github.io/jq/) to extract a single value from the returned json.
