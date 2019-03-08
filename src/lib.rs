#[macro_use]
extern crate ocaml;

use std::mem;

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.custom_ptr_val_mut::<Vec<i32>>();
    mem::drop(ptr);
    println!("Finalize");
}

macro_rules! modify_vec {
    ($v:ident, $vec:ident, $block:block) => {
        let mut $vec = &mut *$v.custom_ptr_val_mut::<Vec<i32>>();

        $block

        mem::forget($vec);
    };
}

caml!(vec_create, |n|, <dest>, {
    let mut vec: Vec<i32> = Vec::with_capacity(n.usize_val());
    let ptr = &mut vec as *mut Vec<i32>;
    mem::forget(vec);
    dest = ocaml::Value::alloc_custom(ptr, finalize);
} -> dest);

caml!(vec_length, |handle|, <dest>, {
    let p = handle.custom_ptr_val::<Vec<i32>>();
    dest = ocaml::Value::usize((*p).len())
} -> dest);

caml!(vec_push, |handle, x|, {
    modify_vec!(handle, vec, {
        vec.push(x.i32_val());
    });
});

caml!(vec_pop, |handle|, <dest>, {
    modify_vec!(handle, vec, {
        dest = match vec.pop() {
            Some(x) => ocaml::Value::some(x),
            None => ocaml::Value::none()
        };
    });
} -> dest);

caml!(vec_clear, |handle|, {
    modify_vec!(handle, vec, {
        vec.clear()
    });
});

caml!(vec_index, |handle, index|, <dest>, {
    modify_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            dest = ocaml::Value::none();
        } else {
            dest = ocaml::Value::some(vec[index.usize_val()].clone())
        }
    });
} -> dest);

caml!(vec_set_index, |handle, index, x|, {
    modify_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            return
        }

        vec[index.usize_val()] = x.i32_val();
    });
});
