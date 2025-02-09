defmodule Solution do
  @spec score_of_string(s :: String.t) :: integer
  def score_of_string(s) do
    arr = String.to_charlist(s)
    Enum.reduce(arr |> tl, {0, arr |> hd} fn ch, {ans, prev} ->
      {ans + abs(ch - prev), ch}
    end) |> elem(0)
  end
end
