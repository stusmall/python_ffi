%module swig_example
%{
/* Includes the header in the wrapper code */
#include "swig_example.h"
%}

%include "stdint.i"
%include "cstring.i"

/* Parse the header file to generate wrappers */
%include "swig_example.h"