defmodule REWASMTest do
  use ExUnit.Case
  doctest REWASM

  test "greets the world" do
    assert REWASM.hello() == :world
  end
end
