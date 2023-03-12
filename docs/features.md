# Robins Toolkit

Because every developer needs a good sidekick.

## Commands

structure: `robin [mode] [params] [file/folder/string]`

### Notes

Example: `robin notes -n school_meeting` create new note file with the name school_meeting.md

- create: a new markdown file in notes folder
- open: Open a note file
- delete: delete note

### Projects

- create new project (ahk/python) arg: [-n, new] type: [-py, -ahk]
  - example: `robin project -n -py
- open project [-o, open]
- view all projects [a, all]

### TODO

- add new todo
- mark as done
- delete todo
- show all todos

### Search

- search file in directory
- search files containing given string

### "GoTo"

- open directory optional arg: [-o, open]
  - example 1: `robin goto -o projects`
  - example 2: `robin goto notes`
- add new directory

### Help

cmd: `robin -h`

## Params
