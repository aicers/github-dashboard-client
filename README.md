# AICE GitHub Dashboard Client

## GraphQL Schema

- github-dashboard-server: 4b2509fbf459d94216f72abacabe94535e6b88da

## Usage

Build and serve with Trunk. Should be running on <http://127.0.0.1:8080>.

```sh
trunk build
trunk serve --proxy-backend="https://localhost:8000/graphql" --proxy-insecure
```
