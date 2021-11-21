# About
This project is used as the limited space I can find to reproduce a SWIG issue I am running into on another larger project.

The goal is to have one core rust library that contains all the logic of the project.  Using [safer_ffi](https://github.com/getditto/safer_ffi) it will automatically produce a C style interface, any needed glue code on the rust side of the FFI boundary and finally a header file.  This is contained in the ffi directory

Then using [SWIG](www.swig.org/) it will produce low level, language specific bindings for each supported language.  Building on top of these low level, machine generated bindings I will produce high level, idiomatic bindings for each language.

I know that python has many other options for importing C libraries but the goal of this project is much larger than python.  I'd like to support as many languages as possible with safe, automatically produced bindings as possible.

I started out with Python but I am hitting issues with getting strings across the FFI barrier.  I've tried tons of different variations of how to format the ptr set on the python side but can't get past the TypeError.  I have a feeling I'm missing something simple, so I'm looking for help.


# How to build

First make sure you have rust installed.  The easiest way is using [rustup](https://rustup.rs/)

You will also need poetry which can be found [here](https://python-poetry.org/docs/)

Finally, you will need SWIG and gcc which are best pulled from the OS's repos.  This project is build with Ubuntu 20.10

I took a few lazy steps and hard coded some python paths so builds might need tweaks on other systems.  I was planning on cleaning up after I got simple test cases working end to end.

The builds are using the [xtask pattern](https://github.com/matklad/cargo-xtask).  Simply run `cargo xtask build` in the root of the repo.

To run the python script change directories to python, run `poetry run python swig_example/__init__.py`  The following output is expected with what is checked in:

```
# poetry run python swig_example/__init__.py
hello from rust
We got one int 5
Traceback (most recent call last):
  File "/home/stusmall/Workspace/Playground/python_ffi/python/swig_example/__init__.py", line 17, in <module>
    test_with_one_str(_str_to_slice("hi"))
  File "/home/stusmall/Workspace/Playground/python_ffi/python/swig_example/__init__.py", line 9, in _str_to_slice
    slice.ptr = input
TypeError: in method 'slice_boxed_uint8_t_ptr_set', argument 2 of type 'uint8_t *'
```

All machine produced python files, header files and c files are checked in to make this easier for to review and find the error without having to run the program.

The SWIG interface file can be found at `ffi/swig_example.i`.  The safer_ffi produced header file can be found at `ffi/swig_example.h`.  The SWIG produced C module can be found at `python/swig_example/swig_example_wrap.c`.   The SWIG produced python file can be found at `python/swig_example/swig_example.py`.  The attempt to call it is at `python/swig_example/__init__.py`


Any and all help or tips will be greatly appreciated.