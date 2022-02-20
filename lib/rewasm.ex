defmodule REWASM do
  @moduledoc """
  Documentation for `REWASM`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> REWASM.hello()
      :world

  """
  def hello do
    :world
  end
end

defmodule REWASM.NIF.Base64 do
  use Rustler, otp_app: :rewasm, crate: "rewasm_nif_base64"

  @spec decode(binary, atom) :: binary
  def decode(_b64, _opt \\ :standard), do: error()

  @spec encode(binary, atom) :: binary
  def encode(_s, _opt \\ :standard), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end
