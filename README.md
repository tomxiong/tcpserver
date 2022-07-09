[![Rust](https://github.com/tomxiong/tcpserver/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/tomxiong/tcpserver/actions/workflows/test.yml)

# tcpserver is a simple TCP response server which can print message at server console and echo it to client.

* How to build it?
```shell
xionggang@DESKTOP-5HGRQV2:~/rust/projects/tcpserver$ cargo build --release
```
* How to use it?
```shell
xionggang@DESKTOP-5HGRQV2:~/rust/projects/tcpserver$ cd target/releases
xionggang@DESKTOP-5HGRQV2:~/rust/projects/tcpserver$ ./tcpserver
```
Any client to execute telnet to the server IP and port 12345
```shell
xionggang@DESKTOP-5HGRQV2:~$ telnet 127.0.0.1 12345
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.

```
* How to quit from client?
Use "exit" to quit this conversions.
```shell
xionggang@DESKTOP-5HGRQV2:~$ telnet 127.0.0.1 12345
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
hello
hello
exit
Connection closed by foreign host.
xionggang@DESKTOP-5HGRQV2:~$

```
or use quit command of telnet to quit this conversions.
```shell
xionggang@DESKTOP-5HGRQV2:~$ telnet 127.0.0.1 12345
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
hello
hello
team 4
team 4
from 994 of team 4
from 994 of team 4
^]
telnet> quit
Connection closed.
xionggang@DESKTOP-5HGRQV2:~$
```
* For more detail, please to see the screenshots like below：
![Screenshot](tcpserver_running_screenshot.gif)