struct _SomeStruct<'a> {
    _unused: &'a str,
}

impl<'a> _SomeStruct<'a> {

    fn _new() -> Self {
        _SomeStruct { _unused: "aoeu" }
    }

    fn _borrow_mut(&'a mut self) {}

    fn _mutate_self(&mut self) {}

    fn _borrow_immut(&self) {}

    fn _borrow_self(&self) -> &Self {
        self
    }

    fn _borrow_mut_with_ref_self(&mut self, _some_struct: &Self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cannot_borrow_immut_because_borrowed_mut() {
        let mut some_struct: _SomeStruct<'_> = _SomeStruct::_new();
        some_struct._borrow_mut();
        some_struct._borrow_immut();
    }

    #[test]
    fn cannot_borrow_mut_because_borrowed_immut() {
        let mut some_struct = _SomeStruct::_new();
        let borrowed_self = some_struct._borrow_self();
        some_struct._borrow_mut_with_ref_self(borrowed_self);
    }
}
