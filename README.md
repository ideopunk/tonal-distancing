**Tonal Distancing**

This tool reports back on word proximity. I decided to build it after writing a story and only belatedly noticing words being overused ("she frowned ... \n ... she frowned"). I also wanted to try out Rocket! 



To-do:
- Full readme

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