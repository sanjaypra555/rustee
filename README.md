#rustee

rust implementation of unix `tee` with more features

```bash
rustee 0.1.0
Pipe stdin to stdout and file

USAGE:
    rt [OPTIONS] [FILE]

ARGS:
    <FILE>    Output file

OPTIONS:
    -a, --append     Append the output to the file rather than overwriting
    -d, --debug      Debug
    -h, --help       Print help information
    -i, --ignore     Ignore the SIGINT signal
    -n, --noenv      Don't consider RUSTEE_MODE environment variable
    -u, --unique     Pipe only unique lines (won't make existing lines unique in the file)
    -V, --version    Print version information
```


`$ yes | rt out.txt` truncate file, output to out.txt

`$ yes | rt -a out.txt` append output to out.txt

`$ yes | rt -au out.txt` append unique lines to out.txt (It will only append lines that are not present in the file)

`RUSTEE_MODE` environment variable can be used for default behavior. Add following in your `~/.bashrc` or `~/.zshrc` file:

`RUSTEE_MODE=a`: same as `-a` option

`RUSTEE_MODE=u`: same as `-u` option

`RUSTEE_MODE=au`: same as `-au` option

`-n` flag can be used while running the program to ignore `RUSTEE_MODE` environment variable

