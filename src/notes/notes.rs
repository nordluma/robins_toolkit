use std::error::Error;

#[allow(dead_code)]
struct NotesCommand {
    cmd: String,
    file_name: Option<String>,
}

#[allow(dead_code, unused_variables)]
impl NotesCommand {
    pub fn open_single_note(file_name: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
