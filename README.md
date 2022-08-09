A simple Worker written in Rust to explore Cloudflare's isolates (reuse, cold starts, fingerprinting, etc).

```
❯ curl https://counter-worker.YOURDOMAIN.workers.dev
Counter: 1
Cold start: true
Datacenter: AMS
```

Use a real browser and reload the page to see isolate reuse in action.

![](demo.gif)
