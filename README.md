# Fileserver_rust

Network I/O plays a vital role in supporting server software. A file server particularly handles files where clients send/receive files from/to remote servers. Authentication of clients to the server is also critical. 

In this project, we will build a file server application that 
1) Authenticate clients using private/public key
2) Deliver/receive files
3) Write to remote files
4) Search text in remote files.

### Search Text

A user can search for specific texts in its files in the remote server:
  - “search -f [FILE NAME] -s [TEXT]”:
    - Lists the sentences that contain the text in that remote file.
  - “search -s [TEXT]”:
    - List the sentences that contain the text in all remote files of that user.
    - List items consists: [File NAME] [SENTENCE].
  - [TEXT] is not a pattern (only alphnaumerics).
  
  
## How to run:
> cargo run --bin server<br>
> cargo run --bin client
