package io.vaan.rebalancer.utils

object MathUtils {
  def roundTo2(x: Double): Double =
    math.round(x * 100.0) / 100.0
}
