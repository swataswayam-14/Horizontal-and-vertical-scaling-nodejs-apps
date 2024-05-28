# Horizontal Scaling Concepts

## Introduction
 - Horizontal scaling is a technique used to increase the capacity of a system by adding more nodes or servers. This is particularly useful in scenarios where the system experiences sudden spikes in traffic or usage.

## Metrics for Autoscaling
 ### Autoscaling can be based on various metrics, including:
    - Incoming Bandwidth: Scaling up or down based on the amount of data being transmitted.
    - Incoming Request Count: Scaling up or down based on the number of incoming requests.
    - Average CPU Usage of the Cluster: Scaling up or down based on the average CPU usage of the cluster.

## example 1: Video Transcoder 
 - Scenario: A video transcoder application that transcodes videos into different formats (e.g., 360p, 720p, 1080p).
 - Metric: Queue length. When the queue length increases, it indicates a need for more servers to process the transcoding tasks.
 - Action: Scale up by starting more servers to process the transcoding tasks.


## example 2: Online Chess Player Game
 - Scenario: An online chess player game that requires persistent connections between users and servers
 - Metric: Number of online users. When the number of online users exceeds a certain threshold, it indicates a need for more servers to handle the connections. As a single node.js process cannot handle more than 20,000 webhook persistent connections
 - Action: Scale up by starting more servers to handle the connections.

## Benefits of Autoscaling: 
 - Efficient Resource Utilization: It ensures that resources are utilized efficiently, reducing waste and costs.
 - Improved Performance: It helps maintain optimal performance by adding or removing nodes as needed.
 - Scalability: It enables systems to scale horizontally, making them more resilient to sudden spikes in traffic or usage

## Implementation: 

### Autoscaling can be implemented using various tools and technologies, including:

 - Containers: Containers are lightweight and easy to start, making them a popular choice for autoscaling.
 - Cloud Services: Cloud services like AWS, Azure, and Google Cloud provide built-in autoscaling features for their instances.
 - Custom Solutions: Custom solutions can be developed using programming languages like Node.js and frameworks like Express.js.