tally; a mental time tracker and pomodoro

built for personal use

requires rust and ruby
alert.sh uses OSX specific commands

```
touch tally.txt
touch logs.txt // if files do not exist
cargo run // to use, will save data to logs.txt
ruby parse.rb // to parse values from the logs.txt into tally.txt (will clear the logs.txt when done)
```
