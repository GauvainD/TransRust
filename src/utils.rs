use graph::Graph;

pub fn plural(i: usize) -> String {
    if i != 1 {
        String::from("s")
    } else {
        String::new()
    }
}

pub fn as_filter<'a, F, S>(filter: F, name: S) -> Box<Fn(&Graph) -> Result<String, ()> + 'a>
where
    F: Fn(&Graph) -> bool + 'a,
    S: Fn(&Graph) -> String + 'a,
{
    Box::new(move |x| if filter(x) { Ok(name(x)) } else { Err(()) })
}

pub fn combine_filters<'a, F, G>(f: F, g: G) -> Box<Fn(&Graph) -> Result<String, ()> + 'a>
where
    F: Fn(&Graph) -> Result<String, ()> + 'a,
    G: Fn(&Graph) -> Result<String, ()> + 'a,
{
    Box::new(move |x| match f(x) {
        Err(_) => g(x),
        Ok(s) => Ok(s),
    })
}

pub fn combine_transfos<'a, F, G>(f: F, g: G) -> Box<Fn(&Graph) -> Vec<Graph> + 'a>
where
    F: Fn(&Graph) -> Vec<Graph> + 'a,
    G: Fn(&Graph) -> Vec<Graph> + 'a,
{
    Box::new(move |x| {
        let mut t = f(x);
        let mut v = g(x);
        t.append(&mut v);
        t
    })
}

pub fn trash_node(_: &Graph) -> Result<String, ()> {
    Ok("TRASH".to_string())
}
