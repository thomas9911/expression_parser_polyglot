defmodule ExpressionParser do
  def parse_and_eval(text) do
    case ExpressionParserNative.parse_and_eval(text) do
      {:ok, value} ->
        Jason.decode(value)

      e ->
        e
    end
  rescue
    ArgumentError -> {:error, "Invalid argument"}
  end
end
