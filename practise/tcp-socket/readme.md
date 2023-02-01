# TCP socket

## Commands

We can use `netcat` or `telnet` to test against the server

- `nc localhost 8080`: Connect to server using netcat
- `telnet localhost 8080`: Connect to server using telnet

## Implementation

- `main_a`: Create an echo server but it is synchronous
- `main_b`: Upgrade to async echo server. Each connection that we open is an independet task
- `main_c`: Send to all participants the messages but in that case all the chat blocked
- `main_d`: Mirror the message just to the receivers
- `main`: Final

## Sources

- [Lily Mara](https://lilymara.xyz/) thots
- [Lily Mara](https://github.com/lily-mara) github
- [Lily Mara](https://www.youtube.com/watch?v=Iapc-qGTEBQ) chat server
