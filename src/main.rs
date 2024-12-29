#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

#[allow(dead_code)]
impl Folder {
    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> Option<File> {
        match self.contents.remove(index) {
            Some(file) => Some(file),
            _ => None,
        }
    }
}
fn main() {}
