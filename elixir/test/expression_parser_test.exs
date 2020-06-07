defmodule ExpressionParserTest do
  use ExUnit.Case

  test "one" do
    assert ExpressionParser.parse_and_eval("1") == {:ok, 1.0}
  end

  test "sin" do
    assert ExpressionParser.parse_and_eval("[sin(1), 1]") == {:ok, [0.8414709848078965, 1.0]}
  end

  test "invalid argument" do
    assert ExpressionParser.parse_and_eval([1, 2, 3]) == {:error, "Invalid argument"}
  end

  test "invalid expression" do
    assert ExpressionParser.parse_and_eval("sin(") ==
             {:error, " --> 1:5\n  |\n1 | sin(\n  |     ^---\n  |\n  = expected expr"}
  end

  test "invalid variable" do
    assert ExpressionParser.parse_and_eval("sin(a*b)") == {:error, "Variable a not found"}
  end

  test "assign" do
    assert ExpressionParser.parse_and_eval("a = 3; b = 2; c = a*b; c + 14") == {:ok, 20.0}
  end
end
