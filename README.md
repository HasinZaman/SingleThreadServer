# SingleThreadServer
## Description
SingleThreadServer is a rust based server framework to create low level servers.

The framework is responsible for receiving, parsing and sending HTTP requests. The user and developers are responsible providing closures that define how to respond to an HTTP request.
## Getting Started

### Prerequisites

The frame requires the following prerequisites in order to function
 * [Rust](https://www.rust-lang.org/tools/install)

### Installation & Compiling

The framework provides a default server implementations in [releases](https://github.com/HasinZaman/SingleThreadServer/releases). The default server implementations allows for basic handling of GET and Head methods; while all other HTTP methods requests return a [405 Method Not Allowed](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/405).

If the default implementation is not sufficient. The repository can be cloned and edited.
```
gh repo clone HasinZaman/SingleThreadServer
```
The server starts when the [```server::start```]() function is invoked. In which, the function must be given closures to handle the required logic for each [HTTP method](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods). This is done using the [```crate::server::method_logic::MethodLogic```](https://www.rust-lang.org/tools/install) struct.

**Example of Sever Start Up in ```main``` Function**
```rust
fn main() {
    let logic: MethodLogic = MethodLogic {
        get: MethodLogic::default_get_logic(),
        head: MethodLogic::default_not_allowed_logic(),
        post: MethodLogic::default_not_allowed_logic(),
        put: MethodLogic::default_not_allowed_logic(),
        delete: MethodLogic::default_not_allowed_logic(),
        connect: MethodLogic::default_not_allowed_logic(),
        option: MethodLogic::default_not_allowed_logic(),
        trace: MethodLogic::default_not_allowed_logic(),
        patch: MethodLogic::default_not_allowed_logic(),
    };

    server::start(logic);
}
```
Afterwards, the code should be compiled into an executable that can run on a the target machine.
### Set Up Environment

The server is almost set up. There still remains two stepping up.

#### Settings.ron

The [RON](https://github.com/ron-rs/ron) is a file that is used to define legal [hosts](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/host)(IP address, Domains, Sub-Domains, etc.), permitted files and the address the server is running on.

**Example of ```settings.ron``` File**
```Rust
    (
    address: "localhost",
    port : 8080,
    paths : {
        "localhost" : (
            path : "tmp",
            allow : [
                "html",
                "css",
                "js",
                "jpg"
            ]
        ),
        "subdomain.localhost" : (
            path : "subdomain",
            allow : [
                "html"
            ]
        ),
    }
)
```

Required Settings
address : Ip address of the server
port : The port the server is hosted on
paths : A dictionary of valid [hosts](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/host)(IP address, Domains, Sub-Domains, etc.). Each host is associated with a path and 

#### Server Paths
The final server must be structured in a certain way in order to properly function. The executable must be placed in the same directory as settings.ron and "source" sub-directory.

The "sources" sub-directory stores all the path directories for each valid domain specified in settings.ron.

**Example of a Possible Server Folder Structure**
```
(root folder)
    ├─ single_thread_server.exe
    ├─ settings.ron
    └─ source
        ├─ folder name
        │   ├─ ...
        │   └─ index.html
        ├─ some_domain_path_0
        │   ├─ ...
        │   └─ index.html
        ├─ ...
        └─ some_domain_path_n
            ├─ ...
            └─ index.html
```
### Starting server

The server can by simply running the executable.

For development debugging. Running the command ```cargo run``` in project folder will run the server. The correct folder structure specified in the previous section is required. The structure is relative the root project folder.

### Debugging

On runtime - the server generates a ```log.txt```. In which, the server is able to log messages in the format
```
--- --- ---
tag     (time_stamp)
message
--- --- ---
```

The frame by default logs the parsed intial connection request, errors and response. Additional logging can be done by calling the ```Server::log``` function in logic closures provided to ```Server::start```.

## Road Map

**Future Plans**
 - Better error handling in default method implementations
    - In which Results are handled with ```match``` rather than ```unwrap```
 - Default error pages
 - HTTPS support
 - TCP Stream timeouts
 - Use the framework to make multi-threaded, async, and pipeline architecture servers

## License
Distributed under the MIT License. See ```LICENSE.txt``` for more information.
