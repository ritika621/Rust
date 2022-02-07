/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
if n < 0
{-1}
else
{(n*(n+1))/2}
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut counter = 0;
    let _i = 0;
    for i in 0..ls.len() {
        if ls[i] >= s && ls[i] <= e {
            counter += 1;
        }
    }    
    counter

}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool{

    if target == [] {
        return true;
    } else {
        let mut length = 0;
        let _i = 0;
        let _j = 0;
        for i in 0..target.len() {
            for j in set{
                if &target[i] == j
                {length = length + 1;}
            }
        }
        if length == target.len()
        {return true;}
        else {return false;}
    }
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
if ls == [] {
    return None;
} else {
    let mut sum = 0.0;
    let mut length = 0.0;
    let _i = 0;
    for i in 0..ls.len() {
        sum = sum + ls[i];
        length = length + 1.0;
    }
    let mean = sum/length;
    return Some(mean);
}  
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/

pub fn to_decimal(lst: &[i32]) -> i32{
    let mut decimal = 0;
    let mut exp = 1;
    let _i = 0;
    for i in 0..lst.len() {
        decimal += lst[lst.len()-1 - i] * exp;
        exp = 2* exp;
    }
    decimal
}


/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
let mut v = Vec::new();
let mut num = n;
let _i = 0;
for i in 2..num+1 {
    while num%i == 0
    {
        v.push(i);
        num = num/i;
    }
}
return v;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
if lst.len() == 0
{ let v:Vec<i32> = Vec::new();
v}
else {
let mut v = Vec::new();
let _i = 0;
for i in 1..lst.len() {
    v.push(lst[i]);
}   
v.push(lst[0]);
return v;}
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
if target.len() == 0
{
    return true;
}
else if s.len() == 0 || target.len() > s.len()
{
return false;
}
else{
let chars: Vec<char> = s.chars().collect();
let tarchars: Vec<char> = target.chars().collect();
let mut count = 1;
let _i = 0;
let _j = 0;
let mut k= 1;
for i in 0..(chars.len() - tarchars.len() + 1)
{
    if chars[i] == tarchars[0]
    {  
        for j in 1..tarchars.len()
        { if tarchars[j] == chars[i + k]
            {
                count = count + 1;
            }
            k = k + 1;
        }
        k = 1;
        if count == tarchars.len()
        { return true;
        }
    }
    count = 1;
}
return false;
}
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
let chars: Vec<char> = s.chars().collect();
if s.len() == 0
{return None;}
let mut temp = 1;
let mut max  = 0;
let mut start = 0;
let mut end = 0;
let _i = 0;
for i in 1..chars.len() 
{
    if chars[i] == chars[i-1]
    {
        temp = temp + 1;
    }
    else 
    {
        if temp > max
        {  
            max = temp;
            end = i;
            temp = 1;
        }
        else {temp = 1;}
    }
    if temp == chars.len()
    { 
        end = i+1;
    }
}
start = end - max;
return Some (&s[start..end]);
}
