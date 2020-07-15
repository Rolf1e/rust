/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut _function: F) -> Vec<U> 
where
    F: FnMut(T) -> U,
{
    let mut vec = vec![];
    for i in input {
        vec.push(_function(i));
    }
    vec

    //input.iter()
        //.map(| x | _function(x))
        //.collect()
}
