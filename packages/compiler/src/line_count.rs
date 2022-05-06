struct ChunkLine {
    line_number: usize,
    counter: usize,
}

struct ChunkLines {
    lines: Vec<ChunkLine>
}

impl ChunkLines {
    fn new () -> Self {
        ChunkLines { lines: Vec::new() }
    }

    fn push_line (&mut self, line_number: usize) {
        let len = self.lines.len() ;
        if let Some(item) = self.lines.get_mut(len - 1) {
            if item.line_number == line_number {
                item.counter += 1;
                return;
            }
        }
        self.lines.push(ChunkLine {counter: 0, line_number})
    }

    fn get_line(&self, offset: usize) -> Option<usize> {
        let mut remain = offset as i64;
        for item in self.lines.as_slice() {
            remain -= item.counter as i64;
            if remain <= 0 {
                return Some(item.line_number)
            }
        }
        None
    }
}

