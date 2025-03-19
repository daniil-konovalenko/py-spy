use std::collections::HashSet;
use std::io::Write;
use super::StackTrace;

pub struct Coverage {
    lines: HashSet<String>,
    pub show_linenumbers: bool,
}

impl Coverage {
    pub fn new(show_linenumbers: bool) -> Coverage {
        Coverage { lines: HashSet::new() , show_linenumbers}
    }

    pub fn increment(&mut self, trace: &StackTrace) -> std::io::Result<()> {
        // convert the frame into a single ';' delimited String
        let lines = trace.frames.iter().rev().map(|frame| {
            let filename = match &frame.short_filename { Some(f) => &f, None => &frame.filename };

            if self.show_linenumbers && frame.line != 0 {
                format!("{} ({}:{})", frame.name, filename, frame.line)
            } else if filename.len() > 0 {
                format!("{} ({})", frame.name, filename)
            } else {
                frame.name.clone()
            }
        }).collect::<Vec<String>>();
        for line in lines {
            self.lines.insert(line);
        }
        Ok(())
    }
    pub fn write(&self, w: &mut dyn Write) -> Result<(), anyhow::Error> {
        for line in &self.lines {
            w.write_all(line.as_bytes())?;
            w.write("\n".as_bytes())?;
        }
        Ok(())
    }
}

