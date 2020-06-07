import pickle
from .expression_parser_python import parse_and_eval as internal_parse_and_eval


def parse_and_eval(text):
    data = internal_parse_and_eval(text)
    return pickle.loads(bytearray(data))