# Front end
run the frontend with `TRUNK_SERVE_PROXY_BACKEND="http://localhost:5000/api" trunk serve`
This proxies every request that starts with `/api` to the backend, so you can make requests to the backend without worrying about CORS issues.
