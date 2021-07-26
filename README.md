# learning-to-fly


## Summary
Teaching birds(triangles) to hunt food(dots) in a 2d world, using a neural net and a genetic algorithm to improve the neural net. 


## Use it

`git clone https://github.com/tobratland/learning-to-fly`

run `cargo build`

from libs/simulation-wasm run `wasm-pack build`

from www run `npm install`

from www run `npm run start`

go to http://localhost:8080 in your browser

Press train to train one whole generation ahead(about 40 seconds of real time is skipped with this, a new generation is created and info about how the previous generation performed is in the console)