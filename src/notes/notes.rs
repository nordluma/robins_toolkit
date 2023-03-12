use std::{fs::create_dir_all, path::Path, process::Command};

use crate::args::{CreateNote, DeleteNote, NoteCommand, NoteSubcommand, OpenNote};

pub fn handle_note_command(note: NoteCommand) {
    let command = note.command;
    match command {
        NoteSubcommand::Create(note) => create_note(note),
        NoteSubcommand::Open(note) => open_note(note),
        NoteSubcommand::Delete(note) => delete_note(note),
        NoteSubcommand::Show => show_all_notes(),
    }
}

fn create_note(note: CreateNote) {
    println!("Creating new note: {:?}", note);
    // TODO: Read from user config path where the note should be created
    let new_path = format!("C:/testfolder/{}", note.name);
    if !Path::new(&new_path).exists() {
        create_dir_all(&new_path).expect("Couldn't create the folder");
    }

    let cmd_to_execute = format!("start {}", new_path);
    Command::new("cmd")
        .args(["/C", cmd_to_execute.as_str()])
        .spawn()
        .expect("failed to execute process");
}

fn open_note(note: OpenNote) {
    println!("Opening note: {:?}", note);
    todo!("Open note")
}

fn delete_note(note: DeleteNote) {
    println!("Deleting note: {:?}", note);
    todo!("Delete Note")
}

fn show_all_notes() {
    todo!("Show all notes")
}
