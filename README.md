# Wasm game of life
Followed the tutorial available on (https://rustwasm.github.io/docs/book/game-of-life/setup.html)
Added some tweaks on it in order to push it a step further

# How to run it
## Clone the repo
Clone the repo anywhere you want on your pc
`git clone https://github.com/reeves-48777/wasm-game-of-life`

## Install dependencies
Before running the code in your browser you will need a few things first.
Obviously you will need rust and cargo installed on your machine so if it hasn't been done yet go ahead, download and run [rustup](https://rustup.rs/). Also make sure build tools are installed, you will need VS build tools (thanks to the VS community installer) for Windows, and `build-essential` or any equivalent for your distribution.

With cargo installed and configure install wasm-pack, it will allow us to build the rust code to web assembly.
```sh
cargo install wasm-pack
```

And after that we will install nodejs. Personnaly I use nvm.
```sh
# for windows
winget install --id=CoreyButler.NVMforWindows  -e

# for linux
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
```

That's it for the dependencies.

## Run the app
First things first we have to transpile rust code to wasm binary.
```sh
wasm-pack build
```
Running this command will do the job for you, please note that I haven't specified any `--target`, it is because it will use the default aka `webpack`. After the build you will be able to find the files in the `pkg` folder at the root of the project. It is only if you want to see how it is build, it is not relevant for the rest.

After that we will go inside the `www` folder located at the root of the project.
```sh
cd www
```

In this folder we will run npm install.
```sh
npm install
```

You may encounter some troubles along the process of running the app as I did, the npm dependencies could be a cause. Webpack was the problem in my case.

So to fix that :
```sh
# list all outdated dependencies
npm outdated
# update all
npm update
# if it didn't work, update dependencies manually
npm install dependencies@latest
```

### Finally
Run `npm run start` to serve the application locally.
The app will be available at `http://localhost:8080/`.

# Tweaks
- [ ] Hashlife algorithm
- [ ] "Infinite" canvas
- [x] Relative zoom on the canvas

# Side notes
There are few other algorithms mentionned in the tutorial, right now I am focusing on the hashlife one.
I will add the other laters :).

Please report any issues that you may encounter, that is not mentionned in this readme.

Have a good day !