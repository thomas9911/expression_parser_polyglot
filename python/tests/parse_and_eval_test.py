import expression_parser_python


def test_works_with_vars_as_strings():
    assert 3 == expression_parser_python.parse_and_eval("1 + 2")
