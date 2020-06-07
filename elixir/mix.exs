defmodule ExpressionParser.MixProject do
  use Mix.Project

  def project do
    [
      app: :expression_parser,
      version: "0.1.0",
      elixir: "~> 1.9",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [expression_parser_native: []],
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.1"},
      {:jason, ">= 0.0.0"}
    ]
  end
end
