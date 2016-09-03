#!/bin/env python2

# Process harfbuzz-sys/src/lib.rs to generate bindings from it.
# First, remove all linebreaks in fn headers from lib.rs using Vim.
#   Procedure to straighten-out all multiline `pub fn` defs:
#    1. Search for: /^\s*pub fn hb_.*[^;]$/
#    2. Do: qqV/;<Enter>J/<Up><Enter>q
#    3. Do: 100@q
# sys-lib.rs file referenced below is the cleaned-up file!
# Warning: Very hacky and ugly code follows!

import re
from itertools import chain, imap
from sys import exit

def main():
    with open("sys-lib.rs") as f:
        source = f.readlines()

    type_pat = re.compile("\\s*pub type hb_(?P<name>[a-z_]+)_t = (?P<expr>.+);")
    enum_pat = re.compile("Enum_Unnamed[0-9]+")
    int_pat = re.compile("u?int[123468]+_t")
    compound_pat = re.compile("(?:\\*const )?(?:Union|Struct)__?hb_[a-z_]+_t")

    flags = []
    enums = []
    ints = []
    compounds = []

    impl_index = {} # snake_case name -> [methods]

    for line in source:
        matches = type_pat.match(line)
        if matches is None:
            continue
        ty_name, expr = matches.group("name", "expr")
        if enum_pat.match(expr):
            if ty_name.endswith("_flags"):
                flags.append((ty_name, ty_name[:-1].upper() + "_", []))
            else:
                enums.append((ty_name, ty_name.upper() + "_", []))
            impl_index[ty_name] = []
        elif int_pat.match(expr):
            ints.append((ty_name, expr))
            impl_index[ty_name] = []
        elif compound_pat.match(expr):
            impl_index[ty_name] = []
            compounds.append(ty_name)

    variant_pat = re.compile("pub const HB_(?P<name>[A-Z_]+): ::libc::c_uint = (?P<value>[0-9]+);")
    fn_pat = re.compile('''    pub fn hb_(?P<name>[a-z_]+)\((?P<args>.*)\) -> (?P<retty>.+);''')

    for line in source:
        variant = variant_pat.match(line)
        if variant:
            name = variant.group("name")
            value = variant.group("value")
            longest_prefix(name,
                           lambda tupl: tupl[1],
                           chain(flags, enums),
                           lambda tupl: tupl[2].append((name[len(tupl[1]):], value)))
            continue

        fn = fn_pat.match(line)
        if fn:
            name = fn.group("name")
            args = fn.group("args")
            retty = fn.group("retty")
            longest_prefix(name,
                           lambda key: key,
                           impl_index.keys(),
                           lambda key: impl_index[key].append((name[len(key)+1:], args, retty)))

    for name, alias in ints:
        print("pub type {0} = {1};".format(to_camel_case(name), alias))
        print_impl(name, impl_index)

    for name, _, variants in enums:
        print("\nenum {0} {{".format(to_camel_case(name)))
        for vname, vval in variants:
            print("    {0} = {1},".format(vname, vval))
        print("}")
        print_impl(name, impl_index)

    for name, _, variants in flags:
        print("\nflags {0}: u32 {{".format(to_camel_case(name)))
        for fname, fval in variants:
            print("    const {0} = {1},".format(fname, fval))
        print("}")
        print_impl(name, impl_index)

    for c in compounds:
        print("\npub struct {0}(*mut hb_{1}_t);".format(to_camel_case(c), c))
        print_impl(c, impl_index)

def to_camel_case(snake_str):
    components = snake_str.split('_')
    return "".join(x.title() for x in components)

def resolve_type(ty, impl_index):
    ty_pat = re.compile("(?P<ptr>\\*(?:const|mut) )?hb_(?P<hb_ty>[a-z_]+)_t")
    ty_matches = ty_pat.match(ty)
    if ty_matches:
        hb_ty = ty_matches.group("hb_ty")
        if impl_index.get(hb_ty) is not None:
            ptr = ty_matches.group("ptr")
            if ptr is None: ptr = ""
            return ptr + to_camel_case(hb_ty)
        else:
            return ty
    else:
        return ty

def print_impl(name, impl_index):
    methods = impl_index.get(name)
    if not methods:
        return

    arg_pat = re.compile("(?P<ident>[a-z_]+): (?P<ty>.+)")
    print("\nimpl {0} {{".format(to_camel_case(name)))
    for method, args_str, retty in methods:
        arg_names = []
        arg_tys = []
        if args_str:
            arg_exprs = args_str.split(', ')
            for e in arg_exprs:
                matches = arg_pat.match(e)
                arg_name, ty = matches.group("ident", "ty")
                arg_names.append(arg_name)
                arg_tys.append(resolve_type(ty, impl_index))
        params = ", ".join([ident + ": " + ty for ident, ty in zip(arg_names, arg_tys)])
        call_args = ", ".join(arg_names)
        print("\n\
    pub fn {method}({params}) -> {retty} {{\n\
        unsafe {{ {inner_call}({inner_args}) }}\n\
    }}".format(method=method,
               params=params,
               retty=resolve_type(retty, impl_index),
               inner_call="hb_" + name + "_" + method,
               inner_args=call_args))
    print("}")

def longest_prefix(string, prefixer_func, prefixer_list, best_match_func):
    max_len = 0
    best_item = None
    for item in prefixer_list:
        prefix = prefixer_func(item)
        if string.startswith(prefix):
            if max_len < len(prefix):
                max_len = len(prefix)
                best_item = item
    if best_item is not None:
        best_match_func(best_item)

if __name__ == "__main__":
    main()
