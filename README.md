# strange-attractors
Generates images of dynamical system attractors

For now, capable of generating a [Clifford attractor](https://blbadger.github.io/clifford-attractor.html) (or [these](http://paulbourke.net/fractals/clifford/) too) or a [Peter De Jong attractor](http://paulbourke.net/fractals/peterdejong/).

```
strange-attractors 0.1.0

USAGE:
    strange-attractors [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --height <HEIGHT>    [default: 1000]
        --help               Print help information
    -s, --steps <STEPS>      [default: 10000]
    -V, --version            Print version information
    -w, --width <WIDTH>      [default: 1000]

SUBCOMMANDS:
    clifford    
    de-jong     
    help        Print this message or the help of the given subcommand(s)
```

This won't do a whole lot without a subcommand.  For example, `clifford`:

```
strange-attractors-clifford 

USAGE:
    strange-attractors clifford [OPTIONS] -a <A> -b <B> -c <C> -d <D>

OPTIONS:
    -a <A>                   
    -b <B>                   
    -c <C>                   
    -d <D>                   
    -h, --height <HEIGHT>    [default: 1000]
        --help               Print help information
    -s, --steps <STEPS>      [default: 10000]
    -w, --width <WIDTH>      [default: 1000]
```

There's also a nice Makefile that will let you generate a basic Clifford attractor as a PPM image to stdout with `make attractor`. However! You can adjust like so:

```bash
make ATTRACTOR=de-jong \
A=1.641 B=1.902 C=0.316 D=1.525 \
MAX_STEPS=100000 \
attractor > test.ppm
```