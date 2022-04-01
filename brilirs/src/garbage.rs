use fxhash::FxHashMap;

pub struct Collector {
    //Map from pointer base address to count
    total: FxHashMap<usize, i32>,
    //Top of stack is the counters for this function
    diff: Vec<FxHashMap<usize, i32>>
}

impl Default for Collector {
  fn default() -> Self {
    Self {
      total: FxHashMap::with_capacity_and_hasher(20, fxhash::FxBuildHasher::default()),
      diff: vec![],
    }
  }
}

impl Collector {
  // To call as soon as entering a function
    pub fn enter(&mut self) {
    let top = FxHashMap::with_capacity_and_hasher(10, fxhash::FxBuildHasher::default());
    self.diff.push(top);
  }

  pub fn increment(&mut self, address: usize) {
    let a = address;
    let top = self.diff.last_mut().unwrap();
    top.insert(a, 1 + if top.contains_key(&a) { top[&a] } else { 0 });
    let tot = &mut self.total;
      tot.insert(a, 1 + if tot.contains_key(&a) { tot[&a] } else { 0 });
  }

    //Returns true if address now has 0 reference
  pub fn decrement(&mut self, address: usize) -> bool {
      let a = address;
      let top = self.diff.last_mut().unwrap();
      let top_new = top.get(&a).unwrap_or(&0) - 1;
      let tot = &mut self.total;
      let tot_new = tot.get(&a).unwrap_or(&0) - 1;
      if tot_new <= 0 {
	  tot.remove(&a);
	  top.remove(&a);
	  true
      } else{
	  top.insert(a, top_new);
	  tot.insert(a, tot_new);
	  false
      }
  }

  /* Resets collector to previous state
  Returns everything with 0 counter */
    pub fn exeunt(&mut self) -> Vec<usize> {
	let top = self.diff.pop().unwrap();
	let mut garbo: Vec<usize> = vec![];
	for (key, val) in top.iter() {
	    let t = self.total.get(key).unwrap();
	    let t_new = t - val;
	    if t_new == 0 {
		garbo.push(*key);
		self.total.remove(key);
	    } else {
		self.total.insert(*key, t_new);
	    }
	}
	garbo
    }
}

/*
Ok so this is the workflow:

For each function,

    Right after it is called, for each pointer argument increment the counter
    for it

    for each assignment (dst <- src) where dst is of type ptr<T>:

        If dst already has value v, decrement counter for v

        Increment the counter of the new value for dst. "Old counter"
        treated as 0 if the key was not bound initially.

    Right before a function exits:

        Call reset. Free all pointers with counter 0 UNLESS we are
        return said pointer.
*/

// So for the return, if a pointer was returned this will be dealt with
// by the caller function which has to assign that pointer to a variable,
// thereby incrementing its count.
