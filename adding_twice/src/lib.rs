use lalgebra_scalar::Scalar;

pub fn add_curry(num: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x: i32| x + num)
}
pub fn twice<T: Scalar<Item = T> + Clone + std::ops::Add<Output = T> + 'static>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>{
    Box::new(move |x| f(T::zero()) + f(T::zero()) + x)
}