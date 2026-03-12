# Frontend

A Rust/WASM frontend for the Machine Learning platform, built with [Yew](https://yew.rs/) and [Pico CSS](https://picocss.com/).

## Running

**Linux / macOS**
```bash
TRUNK_SERVE_PROXY_BACKEND="http://localhost:5000/api" trunk serve
```

**Windows (PowerShell)**
```powershell
trunk serve --proxy-backend="http://localhost:5000/api"
```

Proxies all `/api` requests to the backend — no CORS configuration needed.

## Features

- **Model list** — browse all trained models via the dropdown selector.
- **Train new model** — provide a name (required), select a training data source (existing file or CSV upload), pick an algorithm and its parameters, then submit.
- **Train further** — create a new version of an existing model using its previous training data. Disabled for Logistic Regression models, which do not support incremental training.
- **Predict** — run inference on a specific model version with custom input parameters.
- **Auto-select after training** — after training completes the UI automatically switches to the newly trained model and selects its latest version.
- **Error / success banners** — a frosted-glass banner appears at the top of the page on API errors (red) or after successful training (green). Dismissible.
- **Loading states** — buttons show a spinner and are disabled while a request is in progress.

## Tech stack

| | |
|---|---|
| Framework | [Yew](https://yew.rs/) 0.22 |
| HTTP client | `gloo-net` |
| Styling | [Pico CSS](https://picocss.com/) (classless, cyan theme) |
| Build tool | [Trunk](https://trunkrs.dev/) |
