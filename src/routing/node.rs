use super::middleware::MiddlewareBehaviour;

// will be mapped with something in matcher
pub struct Node(Vec<Box<dyn MiddlewareBehaviour>>);
