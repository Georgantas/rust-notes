// in a closure signature
// x: &u32 => means x is of type &u32
// &x => means "destructure x by &"

// in the below example, knowing that a &&u32 is passed into filter, x becomes type &u32
// because of the destructuring (&& minus & equals &)
t.iter().filter(|&x| x == &5).sum::<u32>();

// in the below example, knowing that &&u32 is passed into filter, we get an ERROR
// because &&u32 is passed in, but the signature expects &u32
t.iter().filter(|x: &u32| x == &5).sum::<u32>();
