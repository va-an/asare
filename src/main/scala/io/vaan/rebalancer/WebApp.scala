package io.vaan.rebalancer

import org.scalajs.dom
import org.scalajs.dom.document
import org.scalajs.dom.html.Input

import scala.scalajs.js.annotation.JSExportTopLevel

object WebApp {
  def main(args: Array[String]): Unit =
    document.addEventListener("DOMContentLoaded", (e: dom.Event) => setupUI())

  def appendPar(targetNode: dom.Node, text: String): Unit = {
    val parNode = document.createElement("p")
    parNode.textContent = text
    targetNode.appendChild(parNode)
  }

  def addClickedMessage(): Unit =
    appendPar(document.body, "You clicked the button!")

  def customClickeMessage(): Unit =
    appendPar(document.body, document.getElementById("asset-1").asInstanceOf[Input].value)

  def setupUI(): Unit = {
//    val button = document.createElement("button")
//    button.textContent = "Click me!"
//    button.addEventListener("click", (e: dom.MouseEvent) => customClickeMessage())
//    document.body.appendChild(button)
  }

  @JSExportTopLevel("calculate")
  def calculate(): Unit = {
    val records = getRecords
    println(records)

    val currentAllocation = calcCurrentAllocation(records)
    println(currentAllocation)

    setDoubleById("allocation-1", currentAllocation(0).allocation)
    setDoubleById("allocation-2", currentAllocation(1).allocation)
  }

  def getRecords: List[Record] = {
    val r1 = Record(
      ticker = getStringById("asset-1"),
      count = getDoubleById("count-1").toInt,
      price = getDoubleById("price-1"),
      allocation = getDoubleById("desired-allocation-1"),
      buyOrSell = None,
      currentAllocation = None
    )

    val r2 = Record(
      ticker = getStringById("asset-2"),
      count = getDoubleById("count-2").toInt,
      price = getDoubleById("price-2"),
      allocation = getDoubleById("desired-allocation-2"),
      buyOrSell = None,
      currentAllocation = None
    )

    List(r1, r2)
  }

  def calcCurrentAllocation(records: List[Record]): List[Record] = {
    val sum = records.map(x => x.price * x.count).sum
    records
      .map { x =>
        val amount = x.price * x.count
        val allocation = amount / sum * 100
        x.copy(allocation = roundTo2(allocation))
      }
  }

  def setDoubleById(id: String, number: Double): Unit =
    document.getElementById(id).asInstanceOf[Input].valueAsNumber = number

  def getStringById(id: String): String =
    document.getElementById(id).asInstanceOf[Input].value

  def getDoubleById(id: String): Double =
    document.getElementById(id).asInstanceOf[Input].valueAsNumber

  def roundTo2(x: Double): Double =
    math.round(x * 100.0) / 100.0
}
