use Op;

struct Iterator {
  ops: Vec<Op>,
  index: usize,
  offset: usize
}

impl Iterator {
  fn new(op:Vec<Op>) -> Self {
    Iterator {
      ops: op,
      index: 0,
      offset: 0
    }
  }

  fn has_next(&self) -> bool {
    self.peek_length() < usize::max_value()
  }

  fn peek(&self) -> Op {
    self.ops[self.index].clone()
  }

  fn next(&mut self, length:Option<usize>) -> Op {
    let mut len = length.unwrap_or(usize::max_value());
    if self.index > self.ops.len() {
      Op::retain(usize::max_value())
    } else {
      let next_op = self.ops[self.index].clone();
      let op_length = Op::len(&next_op).unwrap_or(0);
      if len >= (op_length - self.offset) {
        len = op_length - self.offset;
        self.index += 1;
        self.offset = 0;
      } else {
        self.offset += len
      }

      if Some(next_op.delete) != None {
        Op::delete(len)
      } else {
        let mut retain = Some(0);
        let mut ins:Option<String> = None;
        if Some(next_op.retain) != None {
          retain = Some(len);
        // } else if Some(next_op.insert) != None {
        } else {
          let text = next_op.insert.unwrap_or_else(|| String::from(""));
          if self.offset < len && len < text.len() {
            ins = Some(text[self.offset..len].to_string());
          }
        }
        Op::new(retain, ins, None)
      }
    }
  }

  fn peek_length(&self) -> usize {
    // if our index is in bounds
    if self.ops.len() < self.index && self.index > 0 {
      let op = self.ops[self.index].clone();
      let op_len = Op::len(&op).expect("Op had no value");
      op_len - self.offset
    } else {
      0
    }
  }

  fn rest(&mut self) -> Vec<Op> {
    if !self.has_next() {
      let ops:Vec<Op> = vec![];
      ops
    } else if self.offset == 0 {
      self.ops[..self.index].to_vec()
    } else {
      let old_offset = self.offset;
      let old_index = self.index;
      let next = self.next(None);
      let mut rest = self.ops[..self.index].to_vec();
      rest.insert(0, next);
      self.offset = old_offset;
      self.index = old_index;
      rest
    }
  }
}