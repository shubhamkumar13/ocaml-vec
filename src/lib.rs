use ocaml::Pointer;

#[ocaml::func]
pub fn vec_create(n: ocaml::Int) -> Pointer<'static, Vec<ocaml::Int>> {
    let vec: Vec<ocaml::Int> = Vec::with_capacity(n as usize);
    let mut ptr: Pointer<Vec<ocaml::Int>> = Pointer::alloc(None, None);
    ptr.set(vec);
    ptr
}

#[ocaml::func]
pub fn vec_length(handle: Pointer<Vec<ocaml::Int>>) -> ocaml::Int {
    let p = handle.as_ref();
    p.len() as ocaml::Int
}

#[ocaml::func]
pub fn vec_push(mut handle: Pointer<Vec<ocaml::Int>>, x: ocaml::Int) {
    let p = handle.as_mut();
    p.push(x);
}

#[ocaml::func]
pub fn vec_pop(mut handle: Pointer<Vec<ocaml::Int>>) -> Option<ocaml::Int> {
    let p = handle.as_mut();
    p.pop()
}

#[ocaml::func]
pub fn vec_clear(mut handle: Pointer<Vec<ocaml::Int>>) {
    let p = handle.as_mut();
    p.clear();
}

#[ocaml::func]
pub fn vec_index(handle: Pointer<Vec<ocaml::Int>>, index: ocaml::Int) -> Option<ocaml::Int> {
    let p = handle.as_ref();
    p.get(index as usize).map(|x| *x)
}

#[ocaml::func]
pub fn vec_set_index(mut handle: Pointer<Vec<ocaml::Int>>, index: ocaml::Int, x: ocaml::Int) {
    let p = handle.as_mut();
    p[index as usize] = x;
}
