#![allow(unused_macros, dead_code)]
use cargo_snippet::snippet;

#[snippet(name = "@UnionFind")]
struct UnionFind<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    map:  std::collections::HashMap<T, usize>,
    par:  Vec<T>,
    rank: Vec<usize>,
}

#[snippet(name = "@UnionFind")]
impl<T> UnionFind<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    fn new(init: Vec<T>) -> Self {
        let len = init.len();
        let mut map = std::collections::HashMap::new();
        for (i, v) in init.iter().enumerate() {
            map.insert(v.clone(), i);
        }
        Self {
            map:  map,
            par:  init,
            rank: vec![0; len],
        }
    }

    fn find(&mut self, i: T) -> T {
        let i_ind = {
            if let Some(&ind) = self.map.get(&i) {
                ind
            }
            else {
                panic!("The value can't be found. [in find of UnionFind]");
            }
        };

        if i == self.par[i_ind] {
            return i;
        }
        else {
            let par = self.par[i_ind].clone();
            let tmp = self.find(par);

            self.par[i_ind] = tmp.clone();

            return tmp;
        }
    }

    #[inline]
    fn same(&mut self, i: T, j: T) -> bool { self.find(i) == self.find(j) }

    fn unite(&mut self, i: T, j: T) {
        let i_par = self.find(i);
        let j_par = self.find(j);

        let i_par_ind = {
            if let Some(&ind) = self.map.get(&i_par) {
                ind
            }
            else {
                panic!("The value can't be found. [in unite of UnionFind]");
            }
        };

        let j_par_ind = {
            if let Some(&ind) = self.map.get(&j_par) {
                ind
            }
            else {
                panic!("The value can't be found. [in unite of UnionFind]");
            }
        };

        if self.rank[i_par_ind] > self.rank[j_par_ind] {
            self.par[j_par_ind] = i_par;
        }
        else {
            self.par[i_par_ind] = j_par;
            if self.rank[i_par_ind] == self.rank[j_par_ind] {
                self.rank[j_par_ind] += 1;
            }
        }
    }
}
