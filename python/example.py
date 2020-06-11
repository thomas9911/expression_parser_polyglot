import expression_parser_python

def print_eval(text):
    print(expression_parser_python.parse_and_eval(text))



print_eval("x = 5; y = 123; y*cos(x)")
print_eval("""
p = cos(1.23*pi); 
more_list = [4, 5, 6];
{
    \"test\": p*123,
    \"list\": concat([1, 2, 3], more_list)
}
""")