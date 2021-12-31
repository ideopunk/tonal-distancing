**Tonal Distancing**

This tool reports back on word proximity. I decided to build it after writing a story and only belatedly noticing words being overused ("she frowned ... \n ... she frowned"). I also wanted to try out Rocket!

To-do:

-   Deploy client to access server.

```
USAGE:
    tdist [OPTIONS] <source>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lookahead <Buffer Length>    Set how far ahead to check [default: 50]
    -r, --response <Response Type>     Optional output specification. [values: "raw" | "formatted"] [default: "formatted"]
    -s, --stopwords <Stop Words>       Optional personal stop-word list. Accepts a comma-separated list, or a file path to a line-separated list. If not provided, a default list is used

ARGS:
    <source>    Content to evaluate. Accepts a file path or a string
```
