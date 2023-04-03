

#[derive(Clone, Debug, Default)]
struct Node<T>
{
    val: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T>
{
    pub fn new(val: T) -> Node<T>
    {
        Node { val , next: None }
    }
    pub fn insert(self, val: T) -> Node<T>
    {
        if self.next.is_none()
        {
            Node { val: self.val, next: Some(Box::new(Node { val, next: None})) }
        }
        else
        {
            Node { val: self.val, next: Some(Box::new((*self.next.unwrap()).insert(val)))}
        }
    }
}

impl<T: PartialEq> Node<T>
{
    pub fn contains(self, val: T) -> bool
    {
        if self.next.is_none()
        {
            self.val == val 
        }
        else
        {
            if self.val == val
            {
                true
            }
            else
            {
                (*self.next.unwrap()).contains(val)
            }
        }
    }
}
