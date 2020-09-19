package io.vaan.rebalancer

case class Record(
    ticker: String,
    count: Int,
    price: Double,
    allocation: Double,
    buyOrSell: Option[Double] = None,
    currentAllocation: Option[Double] = None
)
