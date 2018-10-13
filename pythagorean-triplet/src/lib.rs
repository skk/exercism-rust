use std::fmt;

const START: u32 = 1;
const END: u32 = 1001;

#[derive(Debug)]
struct PythagoreanTriple(u32, u32, u32);

impl fmt::Display for PythagoreanTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

fn pythagorean_triples() -> Vec<Option<PythagoreanTriple>> {
    let outer : Vec<u32> = (START..END).into_iter().collect();
    let middle: Vec<u32> = (START..END).into_iter().collect();
    let inner: Vec<u32> = (START..END).into_iter().collect();

    let triples: Vec<Option<PythagoreanTriple>> =
        outer.iter().map(|a|
            middle.iter().map(|b|
                inner.iter().map(|c|
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        return Some(PythagoreanTriple(*a, *b, *c));
                    }
                    else {
                        return None;
                    }
                )
            )
    ).collect();
    triples
//    let t: Vec<PythagoreanTriple> = triples.iter().filter(Some).collect();
//    t
//    for a in START..END {
//        for b in START..END {
//            for c in START..END {
//                if a.pow(2) + b.pow(2) == c.pow(2) {
//                    let triple = PythagoreanTriple(a, b, c);
//                    triples.push(triple);
//                }
//            }
//        }
//    }
//    triples
}


pub fn find() -> Option<u32> {
    let values = pythagorean_triples();
    values.iter().for_each(|val|
        match val {
            Some(pt) => {
                println!("pt {}", pt);
                let a = pt.0;
                let b = pt.1;
                let c = pt.2;
                if a + b + c == 1000 {
                    println!("return a * b * c");
                    Some(a * b * c)
                }
            },
            _ => ()
        }
    );


//    for val in values {
//        let a = val.0;
//        let b = val.1;
//        let c = val.2;
//        if a + b + c == 1000 {
//            return Some(a * b * c);
//        }
//    }

    None
}
