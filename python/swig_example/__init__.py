__version__ = '0.1.0'

from swig_example import test_with_no_args, test_with_one_int, \
    test_with_one_str, slice_boxed_uint8_t


def _str_to_slice(input: str) -> slice_boxed_uint8_t:
    slice = slice_boxed_uint8_t()
    slice.ptr = input
    slice.len = len(input)
    return slice

test_with_no_args()

test_with_one_int(5)

test_with_one_str(_str_to_slice("hi"))
