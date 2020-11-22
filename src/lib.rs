struct _Foo<'a> {
    bar: Option<_Bar<'a>>,
    baz: _Baz,
}

struct _Bar<'a>(&'a _Baz);

struct _Baz;

impl<'a> _Foo<'a> {
    fn _new() -> Self {
        _Foo {
            bar: None,
            baz: _Baz,
        }
    }

    fn _borrow_mut(&'a mut self) {
        self.bar = Some(_Bar(&self.baz));
    }

    fn _borrow_immut(&self) {}

    fn _borrow_baz(&self) -> &_Baz {
        &self.baz
    }

    fn _borrow_mut_with_ref_baz(&mut self, _baz: &_Baz) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cannot_borrow_immut_because_borrowed_mut() {
        let mut foo = _Foo::_new();
        foo._borrow_mut();
        foo._borrow_immut();
    }

    #[test]
    fn cannot_borrow_mut_because_borrowed_immut() {
        let mut foo = _Foo::_new();
        let baz = foo._borrow_baz();
        foo._borrow_mut_with_ref_baz(baz);
    }
}
