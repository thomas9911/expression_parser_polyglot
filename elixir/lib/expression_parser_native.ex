defmodule ExpressionParserNative do
  @moduledoc """
  Documentation for ExpressionParserNative.
  """
  use Rustler, otp_app: :expression_parser, crate: :expression_parser_native

  @doc """

  """
  def parse_and_eval(_), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
