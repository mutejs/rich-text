use Iterator;

#[derive(Default,Clone)]
struct Op {
  delete: Option<usize>,
  insert: Option<String>,
  retain: Option<usize>
}

impl Op {
  fn new (retain:Option<usize>, insert:Option<String>, delete:Option<usize>) -> Self {
    Op {
      retain,
      insert,
      delete
    }
  }

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

  fn insert (text:&str) -> Self {
    Op {
      insert: Some(text.to_string()),
      ..Default::default()
    }
  }

  fn iterator (op:Vec<Op>) -> Iterator {
    Iterator::new(op)

  }

  fn len (op:&Op) -> Option<usize> {
    let default:Option<usize> = Some(0);
    if Some(op.delete) != None {
      op.delete
    } else if Some(&op.insert) != None {
      let ins:Option<usize> = op.insert.as_ref().map(|s| s.len());
      ins
    } else if Some(op.retain) != None {
      op.retain
    } else {
      default
    }
  }
}