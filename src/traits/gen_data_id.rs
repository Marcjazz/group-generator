pub trait GenDataId<T: Sized> {
    fn set_id(&mut self, id: T);
    fn get_id(&self) -> T;
}