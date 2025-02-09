defmodule Solution do
  @spec add_digits(num :: integer) :: integer
  def add_digits(num) when num < 10, do: num

  def add_digits(num) do
    num
    |> Integer.digits()
    |> Enum.sum()
    |> add_digits()
  end
end

