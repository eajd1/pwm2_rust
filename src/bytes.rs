pub trait Bytes {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl Bytes for String {

    fn to_bytes(&self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        return String::from_utf8(bytes.to_vec())
            .expect("Error converting bytes to String");
    }
}

impl Bytes for isize {

    fn to_bytes(&self) -> Vec<u8> {
        return isize::to_le_bytes(8).to_vec();
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let array: [u8; 8] = bytes[..8].try_into().unwrap();
        return isize::from_le_bytes(array);
    }
}

impl Bytes for usize {

    fn to_bytes(&self) -> Vec<u8> {
        return usize::to_le_bytes(8).to_vec();
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let array: [u8; 8] = bytes[..8].try_into().unwrap();
        return usize::from_le_bytes(array);
    }
}

impl<T> Bytes for Vec<T>
where T: Sized + Bytes {

    fn to_bytes(&self) -> Vec<u8> {
        return self.into_iter()
            .map(
                |x| -> Vec<u8> {
                    x.to_bytes().to_vec()
            })
            .reduce(
                |mut l, r| {
                    l.extend(r);
                    return l;
                }                
            ).expect("Error converting bytes");
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut vec: Self = vec![];
        let mut bytes = bytes;
        while bytes.len() > 0 {
            vec.push(T::from_bytes(bytes));
            bytes = &bytes[std::mem::size_of::<T>()..];
        }
        return vec;
    }
}
