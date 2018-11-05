#[derive(Default,Clone)]
struct Op {
  delete: Option<usize>,
  insert: Option<String>,
  retain: Option<usize>
}

impl Op {
  fn delete (len:usize) -> Self {
    Op {
      delete: Some(len),
      ..Default::default()
    }
  }

  fn retain (len:usize) -> Self {
    Op {
      retain: Some(len),
      ..Default::default()
    }
  }

  fn insert (text:String) -> Self {
    Op {
      insert: Some(text.to_string()),
      ..Default::default()
    }
  }

  fn iterator (op:Vec<Op>) -> Iterator {
    Iterator::new(op)

  }

  fn len (op:Op) -> Option<usize> {
    let default:Option<usize> = Some(0);
    if Some(op.delete) != None {
      return op.delete
    } else if Some(op.insert) != None {
      return op.insert.as_ref().map(|s| s.len())
    } else if Some(op.retain) != None {
      return op.retain
    } else {
      return default
    };
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
      let op = self.ops[self.index].clone();
      let opLen = Op::len(op).expect("Op had no value");
      return opLen - self.offset;
    } else {
      return 0
    }
    
  }
}