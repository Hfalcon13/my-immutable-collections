
use core::option::Option::{Some,None};


use crate::traversion_options::TraversionOptions;

#[derive(Clone)]
pub struct Tree<T>
{
    pub left: Option<Box<Tree<T>>>,
    pub val: T,
    pub rigth: Option<Box<Tree<T>>>,
}

impl<T> Tree<T> where T: Clone
{
    pub fn count(self) -> u32
    {
        1
        + 
        if self.left.is_some() 
        {
            (*self.left.unwrap()).count()
        } else { 0 }
        +
        if self.rigth.is_some() 
        {
            (*self.rigth.unwrap()).count()
        } else { 0 }
    }
    pub fn height(self) -> u32
    {
        if self.left.is_none() && self.rigth.is_none(){1}
        else
        {
            std::cmp::max(
            if self.left.is_some()
            {
                (*self.left.unwrap()).height()
            } else { 0 } + 1
            ,
            if self.rigth.is_some()
            {
                (*self.rigth.unwrap()).height()
            } else { 0 } + 1
            ) - 1
        }
    }
    // pub fn efficency(self) -> u32
    // {
    //     let mut sum = 0;
    //     let mut i = 1;
    //     let c = self.clone().count();
    //     while c < sum
    //     {
    //         sum += i * i;
    //         i += 1;
    //     }
    //     self.clone().height() - i - 1
    // }
}

impl<T: Default + Copy> Tree<T>
{
    // O(1)
    pub fn new(val: Option<T>) -> Tree<T>
    {
        Tree { left: None, val: val.unwrap_or_default() , rigth: None }
    }
    // O(1)
    fn _rw_left(self, t: Tree<T>) -> Tree<T>
    {
        Tree { left: Some(Box::new(t.clone())), ..self }
    }
    // O(1)
    fn _rw_rigth(self, t: Tree<T>) -> Tree<T>
    {
        Tree { rigth: Some(Box::new(t.clone())), ..self }
    }
    // O(n)
    pub fn to_vec(self, to: Option<TraversionOptions>, acc: Option<Vec<T>>) -> Vec<T>
    {
        let to = to.unwrap_or_default();
        let acc = acc.unwrap_or_default();
        //default params

        match to 
        {
            TraversionOptions::Thili =>
            {
                let mut temp = vec![self.val];
                if self.left.is_some()
                {
                    temp.con(self.left.unwrap().to_vec(Some(to), Some(acc.clone())));

                }
                if self.rigth.is_some()
                {
                    temp.con(self.rigth.unwrap().to_vec(Some(to), Some(acc.clone())));
                }
                temp
            },
            TraversionOptions::Tohi =>
            {
                let mut temp: Vec<T> = vec![];
                if self.left.is_some()
                {
                    temp.con(self.left.unwrap().to_vec(Some(to), Some(acc.clone())));
                }
                temp.con(vec![self.val]);
                if self.rigth.is_some()
                {
                    temp.con(self.rigth.unwrap().to_vec(Some(to), Some(acc.clone())));
                }
                temp
            },
            TraversionOptions::Sofi =>
            {
                let mut temp: Vec<T> = vec![];
                if self.left.is_some()
                {
                    temp.con(self.left.unwrap().to_vec(Some(to), Some(acc.clone())));
                }
                if self.rigth.is_some()
                {
                    temp.con(self.rigth.unwrap().to_vec(Some(to), Some(acc.clone())));
                }
                temp.con(vec![self.val]);
                temp
            },
        }
    }
}
impl<T: Default + Copy + PartialOrd> Tree<T>
{
    // O(log(n))
    pub fn insert(self, val: T) -> Tree<T>
    {
        if val < self.val
        {
            if self.left.is_none()
            {
                Tree { left: Some(Box::new(Tree::<T>::new(Some(val)))), ..self}
            }
            else
            {
                Tree { left: Some(Box::new((*self.left.unwrap()).insert(val))), ..self}
                //(*self.left.unwrap()).insert(Some(to), val) //buged
            }
        }
        else
        {
            if self.rigth.is_none()
            {
                Tree { rigth: Some(Box::new(Tree::<T>::new(Some(val)))), ..self}
            }
            else
            {
                Tree { rigth: Some(Box::new((*self.rigth.unwrap()).insert(val))), ..self}
                //(*self.rigth.unwrap()).insert(Some(to), val) //buged
            }
        }
    }
    // O(n^3)
    pub fn balance(self) -> Tree<T>
    {
        if self.clone().count() < 2
        { 
            return self.clone();
        }

        let mut bi: Vec<(i32, T)> = vec![];
        let contents = self.clone().to_vec(None, None);
        for i in contents.clone()
        {
            let mut ofset = 0;
            for j in contents.clone()
            {
                if i < j
                {
                    ofset += 1;
                }
                else if j < i
                {
                    ofset -= 1;
                }
            }
            bi.push((ofset,i));
        }

        let mut best: (i32, T) = Default::default();
        for i in bi.clone()
        {
            if i.0.abs() < best.0.abs()
            {
                best = i;
            }
        }

        let mut less: Option<Tree<T>> = None;
        for i in bi.clone() {
            if i.1 < best.1
            {
                if less.is_none()
                {
                    less = Some(Tree::new(Some(i.1)));
                }
                else
                {
                    less = Some(less.unwrap().insert(i.1));
                }
            }
        }

        let mut greater: Option<Tree<T>> = None;
        for i in bi.clone() {
            if i.1 > best.1
            {
                if greater.is_none()
                {
                    greater = Some(Tree::new(Some(i.1)));
                }
                else
                {
                    greater = Some(greater.unwrap().insert(i.1));
                }
            }
        }
        
        return Tree 
        { 
    left: if less.is_none(){None}else{Some(Box::new(less.unwrap().balance()))}, 
    val: best.1, 
    rigth: if greater.is_none(){None}else{Some(Box::new(greater.unwrap().balance()))},
        }
    }
}



trait Concon<T> {
    fn con(&mut self, b: Vec<T>);
}

impl<T> Concon<T> for Vec<T>
{
    // O(n)
    fn con(&mut self, b: Vec<T>)
    {
        for i in b
        {
            self.push(i);
        }
    }
}
