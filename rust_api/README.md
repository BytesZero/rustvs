## 简单测试 Rust API 服务性能

### 运行

``` shell
cargo run
```

### 测试（斐波那契数列）
```
wrk -t12 -c400 -d120s http://127.0.0.1:8080/fibonacci/80
```

### 结果

```
Running 2m test @ http://127.0.0.1:8080/fibonacci/80
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.99ms    4.59ms 137.68ms   89.93%
    Req/Sec     2.56k     1.24k    6.31k    63.54%
  3670936 requests in 2.00m, 381.60MB read
  Socket errors: connect 155, read 102, write 0, timeout 0
Requests/sec:  30564.57
Transfer/sec:      3.18MB
```

- [查看 Node JS 测试结果](https://github.com/yy1300326388/node_api)

## Docker 部署

```
docker build -t rust_api .
docker run -p 8080:8080 rust_api
```

## 对比测试结果
|测试项|Rust|NodeJS|请求量|
|--|--|--|--|
|启动|![Rust](imgs/img5-start-Rust.png)|![NodeJS](imgs/img5-start-Node.png)|没有请求|
|10线程|![Rust](imgs/img5-Rust-t10.png)|![NodeJS](imgs/img5-Node-t10.png)|![](imgs/img5-t10.png)|
|100线程|![Rust](imgs/img5-Rust-t100.png)|![NodeJS](imgs/img5-Node-t100.png)|![](imgs/img5-t100.png)|
|300线程|![Rust](imgs/img5-Rust-t300.png)|![NodeJS](imgs/img5-Node-t300.png)|![](imgs/img5-t300.png)|
|500线程|![Rust](imgs/img5-Rust-t500.png)|![NodeJS](imgs/img5-Node-t500.png)|![](imgs/img5-t500.png)|
|1000线程|![Rust](imgs/img5-Rust-t1000.png)|![NodeJS](imgs/img5-Node-t1000.png)|![](imgs/img5-t1000.png)|
|2000线程|![Rust](imgs/img5-Rust-t2000.png)|![NodeJS](imgs/img5-Node-t2000.png)|![](imgs/img5-t2000.png)|
|3000线程|![Rust](imgs/img5-Rust-t3000.png)|![NodeJS](imgs/img5-Node-t1000.png)|![](imgs/img5-t3000.png)|