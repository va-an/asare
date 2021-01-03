package io.vaan.rebalancer

import io.vaan.rebalancer.utils.JsUtils._
import io.vaan.rebalancer.utils.MathUtils._
import org.scalajs.dom.{Node, Event}
import org.scalajs.dom.document
import org.scalajs.dom.html.Input
import org.scalajs.dom.raw.{Element, HTMLLabelElement, HTMLInputElement}
import cats.implicits._

import scala.scalajs.js.annotation.JSExportTopLevel
import java.util.concurrent.atomic.AtomicInteger

// FIXME: calculate allocation works with 5 rows only
// TODO: only main method must be here
// TODO: create buttons from scala-code
object WebApp {
  val rowCounter = new AtomicInteger(1)

  def main(args: Array[String]): Unit =
    document.addEventListener("DOMContentLoaded", (e: Event) => setupUI())

  def appendPar(targetNode: Node, text: String): Unit = {
    val parNode = document.createElement("p")
    parNode.textContent = text
    targetNode.appendChild(parNode)
  }

  def addClickedMessage(): Unit =
    appendPar(document.body, "You clicked the button!")

  def customClickeMessage(): Unit =
    appendPar(document.body, document.getElementById("asset-1").asInstanceOf[Input].value)

  def setupUI(): Unit = {
    // val button = document.createElement("button")
    // button.textContent = "Click me!"
    // button.addEventListener("click", (e: dom.MouseEvent) => customClickeMessage())
    // document.body.appendChild(button)

    addEmptyRow()
    addEmptyRow()
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

  @JSExportTopLevel("addEmptyRow")
  def addEmptyRow(): Unit = {
    val rowNumber = rowCounter.getAndIncrement()

    // get "rows" element
    val rows = document.getElementById("rows")

    val row = document.createElement("div")
    row.classList.add("row")

    List(
      createColumn(
        labelFor = "asset",
        labelText = s"Asset $rowNumber", 
        inputType = "text",
        rowNumber = rowNumber
      ),

      createColumn(
        labelFor = "count",
        labelText = "Count",
        inputType = "number",
        rowNumber = rowNumber
      ),

      createColumn(
        labelFor = "price",
        labelText = "Price",
        inputType = "number",
        rowNumber = rowNumber
      ),

      createColumn(
        labelFor = "desired-allocation",
        labelText = "Desired allocation",
        inputType = "number",
        rowNumber = rowNumber
      ),

      createColumn(
        labelFor = "current-allocation",
        labelText = "Current allocation",
        inputType = "number",
        rowNumber = rowNumber
      ),

      createColumn(
        labelFor = "buy-or-sell",
        labelText = "Buy or sell",
        inputType = "number",
        rowNumber = rowNumber
      )
    ).map(row.appendChild)

    rows.appendChild(row)
  }

  def createColumn(
    labelFor: String, 
    labelText: String, 
    rowNumber:Int,
    inputType: String
  ): Element = {
    val id = s"$labelFor-$rowNumber"
    val column = document.createElement("div")
    
    // class
    column.classList.add("col-md-2")
    column.classList.add("mb-3")

    // label
    val label = document.createElement("label").asInstanceOf[HTMLLabelElement]
    label.htmlFor = s"$labelFor-$rowNumber"
    label.textContent = labelText
  
    // input
    val input = createInput(inputType, id)

    column.appendChild(label)
    column.appendChild(input)

    column
  }

  def createInput(inputType: String, id: String): HTMLInputElement = {
    val input = document.createElement("input").asInstanceOf[HTMLInputElement]
    input.classList.add("form-control")
    input.id = id
    input.`type` = inputType
    input.required = true
    input.placeholder = ""
    input.value = ""

    input
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
