const express = require('express');
const app = express();

function fibonacci(n) {
    if (n === 0) {
        return 0;
    } else if (n === 1) {
        return 1;
    }

    let a = 0;
    let b = 1;
    for (let i = 2; i <= n; i++) {
        let temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

app.get('/', (req, res) => {
    res.send('Hello, world!');
});

app.get('/fibonacci/:n', (req, res) => {
    const n = parseInt(req.params.n);
    const result = fibonacci(n);
    res.send(`Fibonacci(${n}) = ${result}`);
});

app.listen(8082, () => {
    console.log('Server is running on http://localhost:8082');
});


