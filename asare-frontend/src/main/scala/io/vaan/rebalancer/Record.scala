package io.vaan.rebalancer

case class Record(
    rowId: String,
    ticker: String,
    count: Int,
    price: Double,
    desiredAllocation: Double,
    buyOrSell: Option[Double] = None,
    currentAllocation: Option[Double] = None
)
