#!/bin/zsh
afplay tone.wav
osascript -e 'tell app "System Events" to display alert "Pomodoro Finished"'
