package com.example.grids.model

import androidx.annotation.DrawableRes
import androidx.annotation.StringRes

data class Topic(
    @StringRes val name: Int,
    val availableCources: Int,
    @DrawableRes val imageRes: Int,
)
