# tally; a mental time tracker and pomodoro

built for personal use on the command line

* requires rust and ruby
* alert.sh uses OSX specific commands for playing alert tone

```
touch tally.txt
touch logs.txt 
    // if files do not exist
cargo build --release
./target/release/tally -h
```

todo:
* handle creating log files if they do not exist
