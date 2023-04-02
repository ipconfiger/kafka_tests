# kafka_tests
本质上这就是一个连上获取完meta和topic列表就断开没有任何其他功能的Kafka客户端，用途是用来监测kafka broker是否正常运行。
Essentially, this is a Kafka client that connects to obtain the meta and topic list and then disconnects without any other functions. Its purpose is to monitor whether the Kafka broker is running normally.

### 使用方法 Usage

 ```
 kafka_tests ip:port
 ```