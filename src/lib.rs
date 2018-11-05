struct Op {
  delete: usize,
  insert: String,
  retain: usize
}

impl Op {
  fn iterator (op:Vec<Op>) -> Iterator {
    Iterator::new(op)
  }
  fn len (op:Op) -> usize {
    return match op {
      delete => op.delete,
      insert => op.insert.len(),
      retain => op.retain
    }
  }
}

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

  fn peekLength(&self) -> usize {
    // if our index is in bounds
    if self.ops.len() < self.index && self.index > 0 {
      let op = self.ops[self.index];
      return Op::len(op) - self.offset;
    } else {
      return 0
    }
    
  }
}