# what is module?
# all files and directories are considered to be modules.
# each module exposes only public symbols to other modules.
# put `pub` keyword in front of the symbol to make it public.

# you can import modules like this.
# but if the module name is `core`, it targets the standard library, shipped with the compiler.
# this means that you can't name your own modules `core`.
# also, all import statements must be top-level statements.
use core.printf;

# you can also import all symbols from a module like this.
use core.*;

# you can rename a module to avoid conflicts like this.
use core as my_core;

# some python-style comments!

###
This function does nothing, just for demonstrating doc-comments.
###
fn test_fn() -> void {

}

# global variables are declared like this.
let global_var = 10;

# to make this variable public, put `pub` keyword in front of it.
pub let global_var = 10;

# what about ffi?
# use `expose` keyword to expose foreign functions and global variables to symbol table.
# `expose` keyword implies `pub` keyword.
expose let global_var = 10;

# import foreign functions like this.
# `ref` keyword is required here, because the `external_fn` is actually a reference to the foreign function.
expose ref fn external_fn() -> void;

# export your functions to foreign code like this.
# here, `c_str` is a special type that represents a C string.
# this type must be prefixed with `ref` keyword.
# without `ref` keyword, it is considered as an error.
expose fn exposed_fn(s: ref c_str) -> void {
    printf("Hello, %s!\n", s);
}

pub fn main() -> void {
    printf("Hello, world!\n");
    let n = 10;
    let a = fib_slow(n);
    let b = fib_fast(n);

    # here, the `printf` function can accept variable arguments.
    # you are not allowed to define such functions yourself,
    # but `printf` is a special, built-in function that is allowed to do this. (using internal compiler support)
    printf("fib_slow(%d) = %d\n", n, a);
    printf("fib_fast(%d) = %d\n", n, b);
}

fn fib_slow(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

fn fib_fast(n: u32) -> u32 {
    let a = 0u32;
    let b = 1u32;
    let c = 0u32;

    while n > 0 {
        c = a + b;
        a = b;
        b = c;
        n -= 1;
    }

    return c;
}

struct MyStruct {
    b: bool,
    i: i32,
    ii: i64,
    u: u32,
    uu: u64,
    f: f32,
    ff: f64,
}

fn make_my_struct() -> MyStruct {
    return MyStruct {
        b: true,
        i: 1,
        ii: 2,
        u: 3,
        uu: 4,
        f: 5.0,
        ff: 6.0,
    };
}

fn access_my_struct_by_value(s: MyStruct) -> void {
    printf("b: %b, i: %d, ii: %d, u: %u, uu: %u, f: %f, ff: %f\n", s.b, s.i, s.ii, s.u, s.uu, s.f, s.ff);
}

fn access_my_struct_by_ref(s: ref MyStruct) -> void {
    printf("b: %b, i: %d, ii: %d, u: %u, uu: %u, f: %f, ff: %f\n", s.b, s.i, s.ii, s.u, s.uu, s.f, s.ff);
}

struct MyStructWithRef {
    b: ref bool,
}

fn make_my_struct_with_ref() -> MyStructWithRef {
    let a = true;

    return MyStructWithRef {
        b: ref a, // may cause dangling pointer, but whatever. this is fine.
    };

    return MyStructWithRef {
        b: ref true, // this will cause a compile error; ref keyword must be used to l-value expression.
    };
}

fn access_my_struct_with_ref(s: ref MyStructWithRef) -> void {
    printf("b: %b\n", s.b); // accessing a reference directly will automatically dereference it.
    printf("b: %p\n", ref s.b); // to get the address of the reference, you need to use the ref keyword.
}

# how about array types?
# there are two array types: an array with a fixed, known size, and a slice with a dynamic size.
# the array type is declared like this:
# [type; size]
# ref [type; size]
# the slice type is declared like this (without `ref` keyword is considered as an error):
# ref [type]

# the array will be copied when passed to a function.
fn array_len_by_value(arr: [u8; 5]) -> u32 {
    # it will be 5
    return @arr; # we use @ operator to get the length of the array.
}

# the array will be passed by reference when passed to a function.
fn array_len_by_ref(arr: ref [u8; 5]) -> u32 {
    # it will be 5
    # note that the length of the array is a compile-time constant, so the compiler always replaces @arr with 5.
    # so if the arr passed is malformed (due to forced type conversion), the compiler will not be able to catch it.
    # use slice type to avoid this problem.
    return @arr;
}

fn array_len_by_slice(arr: ref [u8]) -> u32 {
    # it will be dynamic
    return @arr;
}

# how about function types?
# you can use function types only in `ref` context.
# without `ref` keyword, it is considered as an error.
struct StructWithFunction {
    f: ref fn() -> void;
}

fn dummy() -> void {
    return;
}

fn make_struct_with_function() -> StructWithFunction {
    return StructWithFunction {
        f: ref dummy, # don't forget the `ref` keyword, to obtain a reference to the function.
    };
}

fn access_struct_with_function(s: ref StructWithFunction) -> void {
    s.f(); # call the function directly. this is allowed.
}
