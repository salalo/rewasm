defmodule REWASM.MixProject do
  use Mix.Project

  def project do
    [
      app: :rewasm,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.23.0"},
      {:benchwarmer, "~> 0.0.2"}
    ]
  end
end
