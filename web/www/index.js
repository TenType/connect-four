import init, { hello } from '../pkg/connect_four_website.js';

init().then(() => {
    hello('Rust + WebAssembly');
});
