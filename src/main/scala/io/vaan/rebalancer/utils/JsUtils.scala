package io.vaan.rebalancer.utils

import io.vaan.rebalancer.Record
import org.scalajs.dom.document
import org.scalajs.dom.html.Input

object JsUtils {
  def setDoubleById(id: String, doubleValue: Double): Unit =
    document.getElementById(id).asInstanceOf[Input].valueAsNumber = doubleValue

  def getStringById(id: String): Either[Exception, String] = {
    val value = document.getElementById(id).asInstanceOf[Input].value

     Either.cond(
      test = value.nonEmpty,
      right = value,
      left = new Exception("Empty field \"ticker\"")
     )
  }

  def getDoubleById(id: String): Either[Exception, Double] =
    document.getElementById(id).asInstanceOf[Input].value
      .toDoubleOption
      .map(Right(_)).getOrElse(Left(new Exception("Wrong value for double-typed field")))

  def fetchRow(rowNumber: Int): Either[Exception, Record] = {
    for {
      ticker <- getStringById(s"asset-$rowNumber")
      count <- getDoubleById(s"count-$rowNumber")
      price <- getDoubleById(s"price-$rowNumber")
      allocation <- getDoubleById(s"desired-allocation-$rowNumber")
    } yield Record(
      rowId = rowNumber.toString,
      ticker = ticker,
      count = count.toInt,
      price = price,
      desiredAllocation = allocation,
      buyOrSell = None,
      currentAllocation = None
    )
  }
}
