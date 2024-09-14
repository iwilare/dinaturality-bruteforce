use std::{fmt::{self, Display, Formatter}, process::exit};

#[derive(Clone, Debug)]
struct Function {
    tgt: usize,
    vals: Vec<usize>
}

impl Function {
    fn next(mut self) -> Option<Function> {
        for i in 0..self.vals.len() {
            if self.vals[i] >= self.tgt-1 {
                self.vals[i] = 0;
            } else {
                self.vals[i] += 1;
                return Some(self);
            }
        }
        None
    }
    fn functions(src: usize, tgt: usize) -> impl Iterator<Item = Function> {
        let f = Function::new(src, tgt);
        std::iter::successors(Some(f), |f| f.clone().next())
    }
    fn new(src: usize, tgt: usize) -> Function {
        Function { tgt, vals: vec![0; src] }
    }
    fn id(n: usize) -> Function {
        Function { tgt: n, vals: (0..n).collect() }
    }
    fn tgt(&self) -> usize {
        self.tgt
    }
    fn src(&self) -> usize {
        self.vals.len()
    }
    fn compose(&self, g: &Function) -> Function {
        assert!(self.tgt == g.src());
        let mut vals = Vec::with_capacity(g.vals.len());
        for i in 0..g.vals.len() {
            vals.push(g.vals[self.vals[i]]);
        }
        Function { tgt: g.tgt, vals }
    }
    fn from_vec(tgt: usize, vals: Vec<usize>) -> Function {
        assert!(vals.iter().all(|&x| x < tgt));
        Function { tgt, vals }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.tgt == other.tgt && self.vals == other.vals
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // print all values in the array without saving stuff in a string
        write!(f, "[")?;
        for (i, v) in self.vals.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
fn chain(cs: &Vec<&Function>) -> Function {
    let mut f = cs[0].clone();
    for i in 1..cs.len() {
        f = f.compose(&cs[i]);
    }
    f
}

struct CommutativeSquare {
    up_left: Function,
    up_right: Function,
    down_left: Function,
    down_right: Function
}

impl CommutativeSquare {
    fn new(up_left: Function, up_right: Function, down_left: Function, down_right: Function) -> Option<CommutativeSquare> {
        (chain(&vec![&up_left, &up_right]) == chain(&vec![&down_left, &down_right]))
            .then_some(CommutativeSquare { up_left, up_right, down_left, down_right })
    }
    fn up_left(&self) -> &Function { &self.up_left }
    fn up_right(&self) -> &Function { &self.up_right }
    fn down_left(&self) -> &Function { &self.down_left }
    fn down_right(&self) -> &Function { &self.down_right }
}

impl Display for CommutativeSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} ; {} = {} ; {})", self.up_left, self.up_right, self.down_left, self.down_right)
    }
}

fn commutative_squares(n: usize) -> (usize, Vec<CommutativeSquare>) {
    let mut squares = Vec::new();
    let mut i = 0;
    for a in Function::functions(n, n) {
        for b in Function::functions(n, n) {
            for c in Function::functions(n, n) {
                for d in Function::functions(n, n) {
                    match CommutativeSquare::new(a.clone(), b.clone(), c.clone(), d.clone()) {
                        Some(k) => squares.push(k),
                        None => {}
                    }
                    i += 1;
                }
            }
        }
    }
    (i, squares)
}

struct Dinatural<'a> {
    family: [Function; 2],
    s1: &'a CommutativeSquare,
    s2: &'a CommutativeSquare,
}

impl Display for Dinatural<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Functor 1: {}", self.s1)?;
        writeln!(f, "Functor 2: {}", self.s2)?;
        writeln!(f, "Family on 0: {}", self.family[0])?;
        writeln!(f, "Family on 1: {}", self.family[1])
    }
}

impl<'a> Dinatural<'a> {
    fn new(family: [Function; 2], s1: &'a CommutativeSquare, s2: &'a CommutativeSquare) -> Option<Dinatural<'a>> {
          (chain(&vec![&s1.up_left(), &family[0], &s2.up_right()])
        == chain(&vec![&s1.down_left(), &family[1], &s2.down_right()]))
            .then_some(Dinatural { family, s1, s2 })
    }
    fn composes_with(&self, other: &Dinatural) -> bool {
           chain(&self.hexagon_down_other(other)) ==
           chain(&self.hexagon_up_other(other))
    }
    fn hexagon_up_other<'b>(&'b self, other: &'b Dinatural) -> Vec<&'b Function> {
        vec![&self.s1.up_left(), &self.family[0], &other.family[0], &other.s2.up_right()]
    }
    fn hexagon_down_other<'b>(&'b self, other: &'b Dinatural) -> Vec<&'b Function> {
        vec![&self.s1.down_left(), &self.family[1], &other.family[1], &other.s2.down_right()]
    }
}

fn dinaturals_for_squares<'a>(n: usize, p: &'a CommutativeSquare, q: &'a CommutativeSquare) -> Vec<Dinatural<'a>> {
    let mut dinaturals = Vec::new();
    // temporary hack: force the actual dinatural to be the same function
    // for both the top side of the hexagon and the bottom side
    for e1 in Function::functions(n, n) {
        match Dinatural::new([e1.clone(), e1.clone()], p, q) {
            Some(d) => dinaturals.push(d),
            None => {}
        }
    }
    dinaturals
}

fn dinaturals(n: usize, all_squares: &Vec<CommutativeSquare>) -> Vec<Dinatural> {
    let mut dinaturals: Vec<Dinatural> = Vec::new();
    for s1 in all_squares {
        for s2 in all_squares {
            let xs = dinaturals_for_squares(n, s1, s2);
            dinaturals.extend(xs);
        }
    }
    dinaturals
}

fn main() {
    let n = 2;
    let (total, all_squares) = commutative_squares(n);

    println!("Squares, total: {}", total);
    println!("Squares, comm.: {}", all_squares.len());

    let dinaturals = dinaturals(n, &all_squares);
    println!("dinaturals, total: {}", all_squares.len()*all_squares.len()*n.pow(n.try_into().unwrap())*n.pow(n.try_into().unwrap()));
    println!("dinaturals; comm.: {}", dinaturals.len());

    // sanity check that composition is in the right direction
    println!("Comp: {}", chain(&vec![&Function::from_vec(3, vec![0, 1, 1]), &Function::from_vec(3, vec![1, 0, 2])]));

    // find two dinaturals for which the dinaturality condition is not satisfied
    for s1 in all_squares.iter() {
        for s2 in all_squares.iter() {
            for s3 in all_squares.iter() {
                let ds1 = dinaturals_for_squares(n, s1, s2);
                let ds2 = dinaturals_for_squares(n, s2, s3);
                for d1 in ds1.iter() {
                    for d2 in ds2.iter() {
                        if !d1.composes_with(d2) {
                            println!("First dinatural\n------------------");
                            println!("{}", d1);
                            println!("Second dinatural\n------------------");
                            println!("{}", d2);
                            println!("Morphisms exagon up\n------------------");
                            println!("{}", d1.hexagon_up_other(d2).iter().map(|f| format!("{}", f)).collect::<Vec<String>>().join(" ; "));
                            println!("Morphisms exagon down\n------------------");
                            println!("{}", d1.hexagon_down_other(d2).iter().map(|f| format!("{}", f)).collect::<Vec<String>>().join(" ; "));
                            println!("Hexagon up\n------------------");
                            println!("{}", chain(&d1.hexagon_up_other(&d2)));
                            println!("Hexagon down\n------------------");
                            println!("{}", chain(&d1.hexagon_down_other(&d2)));
                            exit(1)
                        }
                    }
                }
            }
        }
    }
}
