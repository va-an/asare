package io.vaan.rebalancer

import io.vaan.rebalancer.utils.JsUtils._
import io.vaan.rebalancer.utils.MathUtils._
import org.scalajs.dom
import org.scalajs.dom.document
import org.scalajs.dom.html.Input
import cats.implicits._

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

    val allocationCalculated = calcCurrentAllocation(records)

    println("current allocation calculated")
    allocationCalculated.foreach(x => println(x.ticker -> x.currentAllocation))

    allocationCalculated.foreach(x =>
      setDoubleById(
        id = s"current-allocation-${x.rowId}",
        doubleValue = x.currentAllocation.get
      )
    )
  }

  // FIXME: looks like shit
  def getRecords: List[Record] = {
    (1 to 5).toList
      .map(fetchRow)
      .filter(_.isRight)
      .map(_.right.get)
  }

  def calcCurrentAllocation(records: List[Record]): List[Record] = {
    val sum = records.map(x => x.price * x.count).sum
    records
      .map { record =>
        val amount = record.price * record.count
        val allocation = amount / sum * 100

        record.copy(currentAllocation = roundTo2(allocation).some)
      }
  }
}
