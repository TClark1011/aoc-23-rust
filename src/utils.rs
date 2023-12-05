pub fn split_vector<T, F>(vec: Vec<T>, predicate: F) -> Vec<Vec<T>>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    let mut result = Vec::new();
    let mut current_chunk = Vec::new();

    for item in vec {
        if predicate(&item) {
            if !current_chunk.is_empty() {
                result.push(current_chunk.clone());
                current_chunk.clear();
            }
        } else {
            current_chunk.push(item);
        }
    }

    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    result
}
