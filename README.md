## OxiBlog

### Overview

OxiBlog is a simple and stupid project made by Rust language for myself. Cargo's workspace is so powerful for teamwork so OxiBlog is seperated to three work spaces which are isolate with each other and loosely coupled.

Besides the teamwork goal, we can easily change technology stack of a space alone not affecting others.
    
- app space
    - contain controllers and handlers
    - implemented by actix-web
- db space
    - contain models
    - implemented by diesel
- views space
    - contain views and templates
    - implemented by askama
    - html is implemented by bootstrap

### Stack
- Rust >= 1.31 edition=2018
- actix-web
- diesel
- postgresql
- askama
- bootstrap

### How to use it
1. make sure your environment has these contents
   - Rust >= 1.31
   - postgresql development tool

2. clone this project to your system

3. configuring the database
    ```shell
    > cd db
    ```
    write your database url in .env file.
    ```shell
    > diesel setup
    > diesel migration run
    ```

4. insert the seed data
    ```shell
    > cd ..
    > cargo run -p db --bin seed
    ```
    Root user, root category, first article will be added by this step.

5. run the application
    ```shell
    > cargo run -p app
    ```

6. open your web browser typing the address 127.0.0.1:3000


---

### Pressure Test

###### configuration
- vitural-box on an old laptop
- System: centos7.5
- CPU: i5 5200u 2core 2.2GHz
- Memory: 2048MB

###### other web site:
```shell
./wrk -t1 -c100 -d30s https://www.baidu.com
Running 30s test @ https://www.baidu.com
  1 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.37s   342.81ms   2.00s    59.81%
    Req/Sec    56.70     29.51   161.00     67.41%
  1544 requests in 30.02s, 23.20MB read
  Socket errors: connect 0, read 0, write 0, timeout 382
Requests/sec:     51.43
Transfer/sec:    791.39KB
```
###### page without datebase query:
```shell
./wrk -t1 -c100 -d30s http://127.0.0.1:3000/users/login
Running 30s test @ http://127.0.0.1:3000/users/login
  1 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    27.58ms    3.52ms  46.37ms   69.62%
    Req/Sec     3.60k   103.62     3.76k    74.32%
  107771 requests in 30.08s, 286.14MB read
Requests/sec:   3582.43
Transfer/sec:      9.51MB
```
```shell
./wrk -t1 -c500 -d30s http://127.0.0.1:3000/users/login
Running 30s test @ http://127.0.0.1:3000/users/login
  1 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   167.16ms   18.42ms 440.49ms   70.86%
    Req/Sec     2.99k   280.14     3.56k    66.10%
  89371 requests in 30.08s, 237.28MB read
Requests/sec:   2971.17
Transfer/sec:      7.89MB
```
###### home page with database query:
```
./wrk -t1 -c100 -d30s http://127.0.0.1:3000
Running 30s test @ http://127.0.0.1:3000
  1 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    91.85ms   21.05ms 200.98ms   74.32%
    Req/Sec     1.09k   213.13     1.81k    70.27%
  32502 requests in 30.03s, 118.50MB read
Requests/sec:   1082.33
Transfer/sec:      3.95MB
```

### Screenshot
##### Home Page
![avatar](./README/20181211173254.png)
##### Blog Page
![avatar](./README/20181211173256.png)
##### Search Page
![avatar](./README/20181211173227.png)
##### User Login Page
![avatar](./README/20181211173651.png)
##### New Article Page
![avatar](./README/20181211173655.png)
##### Edit Article Page
![avatar](./README/20181211173665.png)


### Contribution
You are welcome to post any issues or pull-requests. Since my native language is not English, if there are any grammar errors, just contact me.
