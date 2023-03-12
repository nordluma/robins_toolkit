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
    todo!("Create new Note")
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
