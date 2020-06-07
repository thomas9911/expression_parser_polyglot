defmodule ExpressionParserNativeTest do
  use ExUnit.Case
  doctest ExpressionParserNative

  test "one" do
    assert ExpressionParserNative.parse_and_eval("1") == {:ok, "1.0"}
  end

  test "invalid argument" do
    catch_error(ExpressionParserNative.parse_and_eval([1, 2, 3]))
  end

  test "invalid expression" do
    assert ExpressionParserNative.parse_and_eval("sin(") ==
             {:error, " --> 1:5\n  |\n1 | sin(\n  |     ^---\n  |\n  = expected expr"}
  end

  test "invalid variable" do
    assert ExpressionParserNative.parse_and_eval("sin(a*b)") == {:error, "Variable a not found"}
  end

  test "sin" do
    assert ExpressionParserNative.parse_and_eval("[sin(1), 1]") ==
             {:ok, "[0.8414709848078965,1.0]"}
  end
end
