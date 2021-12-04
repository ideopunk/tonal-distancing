**Tonal Distancing**

This tool reports back on word proximity. 

To-do:
- Full readme
- Weird apostrophe stuff
- Write to stdout instead of to file.
- Tests
- cli rename

```
USAGE:
    tonal-distancing [OPTIONS] <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lookahead <Buffer Length>    Set how far ahead to check [default: 50]
    -s, --stopwords <Stop Words>       Include a path to a line-break-separated text file of words to ignore. A default list is included.

ARGS:
    <path>    Name of file
```