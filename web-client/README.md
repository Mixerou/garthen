# Garthen Web Client

Web Client module of the Garthen Project

## Available Commands

```bash
# Setup module
$ npm install

# Start development server
$ npm run dev

# Build the application for production
$ npm run build

# Locally preview production build
$ npm run preview

# Check code
$ npm run lint

# Fix code
$ npm run lint:fix
```

## Environment Variables

| Variable                      | Default Value | Description                                             |
|-------------------------------|:-------------:|---------------------------------------------------------|
| `NUXT_PUBLIC_GLOBAL_API_PATH` |       -       | Path to the `Global API` module without `/` at the end. |
| `NUXT_PUBLIC_GLOBAL_WS_URI`   |       -       | URI to the `Global WebSocket` module.                   |
