# Multi-Producer, Any-Consumer Channel

Multiple implementations of a concurrent queue using a channel-like API. Multiple producers can push data to multiple consumers, with only one of them receiving any specific value.

## Implementations

### Version 1

Implemented as a `Vec`, with sender/receiver count management through `Drop` and `Clone`, safe access through a single `Mutex` and shared with `Arc`. Receivers busy-wait (with `sleep`) until there is an item in the queue, or the queue is empty and the sender count is 0. Senders add to the queue if the receiver count is >0.

