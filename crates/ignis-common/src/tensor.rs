#[derive(Debug)]
pub struct TensorData<T> {
    pub data: Vec<T>,
    pub n: usize,
    pub ndim: usize,
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
}

impl<T: Clone> TensorData<T> {
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        Self {
            data: data.clone(),
            n: data.len(),
            ndim: shape.len(),
            shape: shape.clone(),
            strides: TensorData::<T>::compute_strides(&shape.as_slice()),
        }
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for TensorData<T> {
    fn from(data: [T; N]) -> Self {
        let shape = vec![N];
        Self {
            data: data.into(),
            n: N,
            ndim: 1,
            shape: shape.clone(),
            strides: TensorData::<T>::compute_strides(&shape.as_slice()),
        }
    }
}

impl<T: Clone, const N: usize, const M: usize> From<[[T; N]; M]> for TensorData<T> {
    fn from(data: [[T; N]; M]) -> Self {
        let flattened: Vec<T> = data.concat();
        let shape = vec![M, N];
        Self {
            data: flattened,
            n: N * M,
            ndim: 2,
            shape: shape.clone(),
            strides: TensorData::<T>::compute_strides(&shape.as_slice()),
        }
    }
}

impl<T: Clone, const N: usize, const M: usize, const P: usize> From<[[[T; N]; M]; P]>
    for TensorData<T>
{
    fn from(data: [[[T; N]; M]; P]) -> Self {
        let flattened: Vec<T> = data
            .iter()
            .flat_map(|m| m.iter())
            .flat_map(|n| n.iter())
            .cloned()
            .collect();

        let shape = vec![P, M, N];
        Self {
            data: flattened,
            n: N * M * P,
            ndim: 3,
            shape: shape.clone(),
            strides: TensorData::<T>::compute_strides(&shape.as_slice()),
        }
    }
}
impl<T> TensorData<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }
}

#[cfg(test)]
mod test {
    use crate::tensor::TensorData;

    #[test]
    fn tensor_data_shape() {
        let lhs: TensorData<i32> =
            TensorData::from([[[1, 2, 3], [3, 4, 3]], [[1, 2, 3], [1, 2, 3]]]);
        println!("{:?}", lhs);
        assert_eq!(lhs.shape, vec![2, 2, 3])
    }
}
