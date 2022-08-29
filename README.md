<p align="center">
  <img src="./src-tauri/icons/128x128@2x.png" width="90">
</p>
<h3 align="center">Pisano</h3>
<p align="center">
Explore interesting Pisano Period visualisations.
</p
<br/>

![Screenshot of the app, showing a grid of circular pisano period visualizations](./Screenshot-light.png#gh-light-mode-only)
![Screenshot of the app, showing a grid of circular pisano period visualizations](./Screenshot-dark.png#gh-dark-mode-only)

## About

Pisano lets you explore different visualizations of [Pisano periods], repeating integer sequences closes related to [Fibonacci numbers].

Pisano Periods are integer sequences derived by taking other integer sequences modulo some constant ( $n$ ). For the Fibonacci numbers ( $0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368$ ) and modulo $n = 3$ the corresponding pisano period starts like this:

<div>

$$ 0, 1, 1, 2, 0, 2, 2, 1, 0, 1, 1, 2, 0, 2, 2, 1, 0, 1, 1, 2, 0, 2, 2, 1, 0, $$

</div

If you then take $n$ equally spaced points on a circle a draw lines between them according to the pisano period you get this visualisation:

<img src="./circle-plot-fib-mod-3.svg" width="300">

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
