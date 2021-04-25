package io.vaan.asare.backend.utils

object NumUtils {
  implicit class NumSyntax(x: Double) {
    def roundTo2: Double =
      math.round(x * 100.0) / 100.0
  }
}
