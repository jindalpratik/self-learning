package com.example.dessertclicker.ui

import androidx.annotation.DrawableRes
import com.example.dessertclicker.data.Datasource.dessertList

data class DessertUiState(
    val dessertsSold: Int = 0,
    val revenue: Int = 0,
    val currentDessertIndex: Int = 0,
    val currentDessertPrice: Int = dessertList[currentDessertIndex].price,
    @DrawableRes val currentDessertImage: Int = dessertList[currentDessertIndex].imageId
)
