## 简单测试 Node JS API 服务性能

### 运行

``` yaml
npm i
npm start
```

### 测试（斐波那契数列）
```
wrk -t12 -c400 -d120s http://127.0.0.1:8081/fibonacci/80
```

### 结果

```
Running 2m test @ http://127.0.0.1:8081/fibonacci/80
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   145.68ms   45.73ms 598.98ms   81.17%
    Req/Sec   138.54     67.60   434.00     64.53%
  198532 requests in 2.00m, 49.42MB read
  Socket errors: connect 155, read 110, write 0, timeout 0
Requests/sec:   1653.05
Transfer/sec:    421.33KB
```

- [查看 Rust 测试结果](https://github.com/yy1300326388/rust_api)

## Docker 部署

```
docker build -t node_api .
docker run -p 8081:8081 node_api
```

## Docker 测试结果

![未测试](imgs/img2.png)
![测试中](imgs/img1.png)


