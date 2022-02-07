use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
    self.push(ele);
       let mut new_child: i32 = (self.len() - 1) as i32;
             let mut parent: i32 = (new_child -1)/2;
       while new_child >  0
       {
      
       
       if self[new_child as usize] >= self[parent as usize] {break}
       self.swap(new_child as usize, parent as usize);
       
       new_child = parent;
   parent = (new_child - 1)/2;
       }
       
    }
    

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {

if self.is_empty() {None}
else {
let mut length = self.len();
self.swap(0, length - 1);
let smallest = self.remove(length-1);

let mut location = 0;
let mut c1 = 0;
let mut c2 = 0;

while location < self.len()/2
{
c1 = (2 * location) + 1;
c2 = (2 * location ) + 2;
if c2 < self.len() && self[c1 as usize] > self[c2 as usize]
{c1 = c2;}
if self[location as usize] < self[c1 as usize] {break}
self.swap(location as usize, c1 as usize);
location = c1 as usize;
}
Some(smallest)
        
    }
    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {None}
	else
	{ Some(&self[0])
    }
}

}
/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let mut sum = p2.0 - p1.0;
    if sum < 0 {sum *= -1;}
    let add  = p2.1 - p1.1;
    if add > 0 { sum += add;}
    else
    {sum = sum + (add * -1);}
    sum
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    let mut prio_q = Vec::new();
    let mut s_enemies = Vec::new();
    let mut allies_done = Vec::new();
    let mut a = Vec::new();
    let mut a1 = Vec::new();
      for (key, val) in enemies.iter(){
      let mut n = Node{
      priority: distance(allies[&String::from("Stark")], *val),
      data: key
   };
   
s_enemies.enqueue(n);

}
   for(key, val) in allies.iter(){
   for(k, v) in enemies.iter(){
if *key == &String::from("Stark"){
   let mut n = Node{
    priority: distance(*val, *v),
    data: key
};
prio_q.enqueue(n);}
else {
let mut n = Node{
priority:distance(*val, *v),
data:k
};
prio_q.enqueue(n);}
a.push(key);



}
}

let mut i = 0;
loop{
let mut ally = prio_q.peek();
match ally{
      Some(d) => {
      if *d.data != &String::from("Stark")
      { if allies_done.contains(d.data) != true && a1.contains(a[i])!= true{
      allies_done.push(d.data);
      a1.push(a[i]);}
      prio_q.dequeue();}
      else {break;}
      },
      
      None => break,
}
i = i + 1;
}


let dist = prio_q.peek();
match dist{
Some(d) => {

	   loop{
	   
	   let mut stark_dist = s_enemies.peek();
	   match stark_dist{
	   	 Some(s) => {print!("{}", s.data);
		 if allies_done.contains(s.data) 
		 	    {s_enemies.dequeue();
			    }
			    else {break;}
			    },
		None => {break;},
		}
		}
		}
		None => (),
			    



}


let return_val = s_enemies.peek();
match return_val{
Some(r) => { 
let coord = enemies[r.data];
	   match coord{
	   	 (a,b) => (r.data,a,b)
		 }
		 },
None => (&"i", 0,0),
}
}






