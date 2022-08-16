# pisano

Pisano lets you explore different visualizations of [Pisano periods], repeating integer sequences closes related to [Fibonacci numbers].

![Screenshot of the app, showing a grid of circular pisano period visualizations](./Screenshot-light.png#gh-light-mode-only)
![Screenshot of the app, showing a grid of circular pisano period visualizations](./Screenshot-dark.png#gh-dark-mode-only)

## Disclaimer

I made this while I was supposed to work on a talk. **This is a procrastination project!** There is much work left here, including bug fixes, adding new sequences, adding new visualizations etc. but let's hope for my talk's sake I will not implement these features! If you are interested in adding these, feel free to open a PR and I will be happy to merge it!

## Contributing

This project uses [pnpm] as its package manager and [sveltekit] as its frontend framework. 

### Developing

Once you have installed the dependencies with `pnpm install` you can start the app in development mode:

```shell
pnpm dev
```

### Building

To create a production version of your app:

```bash
pnpm build
```

## License

[MIT © Jonas Kruckenberg.](./LICENSE)

[pisano periods]: https://en.wikipedia.org/wiki/Pisano_period
[fibonacci numbers]: https://en.wikipedia.org/wiki/Fibonacci_number
[pnpm]: https://pnpm.io
[sveltekit]: https://kit.svelte.dev/
