use graph::Graph;
use graph::format::from_g6;
use graph::nauty::canon_graph;
use std::fs::File;
use std::io::{stdout, BufRead, BufWriter, Write};
use rayon::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use std::sync::Arc;
use utils::*;

pub fn apply_filters<F>(g: &Graph, ftrs: Arc<F>) -> Result<String, ()>
where
    F: Fn(&Graph) -> Result<String, ()>,
{
    ftrs(&g)
}

/// Applying transformations to the graph g.
pub fn apply_transfos<F>(g: &Graph, trs: Arc<F>) -> Vec<Graph>
where
    F: Fn(&Graph) -> Vec<Graph>,
{
    trs(&g).iter().map(|x| canon_graph(x).0).collect()
}

/// Should apply a set of transfomation, filter the graphs and return the result
pub fn handle_graph<F, T>(g: Graph, t: &mut Sender<String>, trsf: Arc<F>, ftrs: Arc<T>)
where
    F: Fn(&Graph) -> Vec<Graph>,
    T: Fn(&Graph) -> Result<String, ()>,
{
    let r = apply_transfos(&g, trsf);
    for h in r {
        let s = apply_filters(&h, ftrs.clone());
        if s.is_ok() {
            t.send(format!("{},{}\n", g, s.unwrap())).unwrap();
        }
    }
}

/// Should apply a set of transfomation, filter the graphs and return the result
pub fn handle_graphs<F, T>(v: Vec<Graph>, t: Sender<String>, trsf: Arc<F>, ftrs: Arc<T>)
where
    F: Fn(&Graph) -> Vec<Graph> + Send + Sync,
    T: Fn(&Graph) -> Result<String, ()> + Send + Sync,
{
    v.into_par_iter()
        .for_each_with(t, |s, x| handle_graph(x, s, trsf.clone(), ftrs.clone()));
}

/// Read files of graphs
/// (file of sigs)
pub fn read_graphs<F>(rdr: &mut F, batchsize: usize) -> Vec<Graph>
where
    F: BufRead,
{
    let mut t = Vec::with_capacity(batchsize);
    for l in rdr.lines().by_ref().take(batchsize) {
        match l {
            Ok(sig) => match from_g6(&sig) {
                Ok(g) => {
                    t.push(g);
                }
                Err(e) => {
                    eprintln!("Wrong input : {}", e);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    t
}

pub fn output(receiver: Receiver<String>, filename: String, buffer: usize) {
    let mut bufout: Box<Write> = match filename.as_str() {
        "-" => Box::new(BufWriter::with_capacity(buffer, stdout())),
        _ => Box::new(BufWriter::with_capacity(
            buffer,
            File::open(filename).expect("Could not open file"),
        )),
    };
    let start = Instant::now();
    let mut i = 0;
    for t in receiver.iter() {
        i += 1;
        bufout.write(&t.into_bytes()).unwrap();
    }
    let duration = start.elapsed();
    eprintln!("Done : {} transformation{}", i, plural(i));
    let secs = duration.as_secs() as usize;
    let millis = (duration.subsec_nanos() as usize) / (1e6 as usize);
    eprintln!(
        "Took {} second{} and {} millisecond{}",
        secs,
        plural(secs),
        millis,
        plural(millis)
    );
}
