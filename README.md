# Time Tracker
Rust command line utility to track time spent on various projects. Mainly a vehicle to start learning Rust and to play with git for source control.
## Installation
No binaries, just clone project and run.
## Usage
```
timer start/stop/status/hours/weeks
```
timer start. Starts a timer by writing the current time and the project in a CSV file.

timer stop reads the timer file and logs the start, stop, duration and project to a separate CSV file. 

timer status - prints current timer status

timer hours - prints all logged hours

timer weeks - prints all weekly totals

## Flags
 - -p PROJECT Specify a project. If no project is specified, the program will just "default"

 - -d DIRECTORY Specify a directory for the files to be stored in. If the directory is on dropbox or some other synchronised location, then this allows projects to be used across systems. This overrides the $TIMERDIR environment variable which can also be used to specify a timer directory.

## TODO
### Test across machines. Currently tested on Linux and Mac.
### All the program currently does is read the log file and print out the contents. Need to add per project totals. Also should be able to do this on a weekly, monthly and user definable time period.
### Loads of other things.

