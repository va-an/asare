package io.vaan.rebalancer.utils

import org.scalajs.dom.document
import org.scalajs.dom.html.Input

object JsUtils {
  def setDoubleById(id: String, number: Double): Unit =
    document.getElementById(id).asInstanceOf[Input].valueAsNumber = number

  def getStringById(id: String): String =
    document.getElementById(id).asInstanceOf[Input].value

  def getDoubleById(id: String): Double =
    document.getElementById(id).asInstanceOf[Input].valueAsNumber
}
