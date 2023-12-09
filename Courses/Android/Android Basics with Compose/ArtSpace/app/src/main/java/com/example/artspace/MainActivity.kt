package com.example.artspace

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.artspace.ui.theme.ArtSpaceTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            ArtSpaceTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    ArtSpaceApp()
                }
            }
        }
    }
}

@Composable
fun ArtSpaceApp() {
    var imageAssetBase by remember { mutableIntStateOf(1) }
    var imageAssetId = painterResource(id = R.drawable.image_asset_1)
    var imageAssetTitleId = stringResource(id = R.string.image_asset_1_title)
    var imageAssetArtistId = stringResource(id = R.string.image_asset_1_artist)
    var imageAssetYearId = stringResource(id = R.string.image_asset_1_year)
    val totalImages = 3

    when(imageAssetBase) {
        1 -> {
            imageAssetId = painterResource(id = R.drawable.image_asset_1)
            imageAssetTitleId = stringResource(id = R.string.image_asset_1_title)
            imageAssetArtistId = stringResource(id = R.string.image_asset_1_artist)
            imageAssetYearId = stringResource(id = R.string.image_asset_1_year)
        }
        2 -> {
            imageAssetId = painterResource(id = R.drawable.image_asset_2)
            imageAssetTitleId = stringResource(id = R.string.image_asset_2_title)
            imageAssetArtistId = stringResource(id = R.string.image_asset_2_artist)
            imageAssetYearId = stringResource(id = R.string.image_asset_2_year)
        }
        3 -> {
            imageAssetId = painterResource(id = R.drawable.image_asset_3)
            imageAssetTitleId = stringResource(id = R.string.image_asset_3_title)
            imageAssetArtistId = stringResource(id = R.string.image_asset_3_artist)
            imageAssetYearId = stringResource(id = R.string.image_asset_3_year)
        }
    }

    fun onClickPrevious() {
        if(imageAssetBase > 1) {
            imageAssetBase--
        } else {
            imageAssetBase = totalImages
        }
    }

    fun onClickNext() {
        if(imageAssetBase < totalImages) {
            imageAssetBase++
        } else {
            imageAssetBase = 1
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(all = 10.dp),
    ) {
        Column(
            modifier = Modifier
                .weight(7f)
                .fillMaxSize(),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            ArtSpaceArtworkImage(
                imageAssetId = imageAssetId,
                imageAssetTitleId = imageAssetTitleId,
                modifier = Modifier
                    .fillMaxWidth()
            )
        }
        Column(
            modifier = Modifier
                .weight(2f)
                .fillMaxSize(),
            verticalArrangement = Arrangement.Bottom
        ) {
            ArtSpaceArtworkDescription(
                imageAssetArtistId = imageAssetArtistId,
                imageAssetTitleId = imageAssetTitleId,
                imageAssetYearId = imageAssetYearId,
                modifier = Modifier
                    .fillMaxWidth()
            )
            Spacer(modifier = Modifier.height(20.dp))
            ArtSpaceChangeArtworkRow(
                onClickPrevious = { onClickPrevious() },
                onClickNext = { onClickNext() },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = 10.dp, start = 20.dp, end = 20.dp)
            )
        }
    }
}

@Composable
fun ArtSpaceArtworkImage(
    imageAssetId: Painter,
    imageAssetTitleId: String,
    modifier: Modifier = Modifier
) {
    Image(
        painter = imageAssetId,
        contentDescription = imageAssetTitleId,
        modifier = modifier.padding(all = 30.dp)
    )
}

@Composable
fun ArtSpaceArtworkDescription(
    imageAssetTitleId: String,
    imageAssetArtistId: String,
    imageAssetYearId: String,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Column(
            modifier = Modifier
                .background(color = MaterialTheme.colorScheme.secondaryContainer)
                .padding(20.dp)
        ) {
            Text(
                text = imageAssetTitleId,
                color = MaterialTheme.colorScheme.onSecondaryContainer,
                fontSize = 25.sp,
                fontWeight = FontWeight.Light
            )
            Row {
                Text(
                    text = imageAssetArtistId,
                    color = MaterialTheme.colorScheme.onSecondaryContainer,
                    fontWeight = FontWeight.Bold
                )
                Spacer(modifier = Modifier.width(5.dp))
                Text(
                    text = imageAssetYearId,
                    color = MaterialTheme.colorScheme.onSecondaryContainer,
                    fontWeight = FontWeight.Light
                )
            }
        }
    }
}

@Composable
fun ArtSpaceChangeArtworkRow(
    onClickPrevious: () -> Unit,
    onClickNext: () -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier.fillMaxSize(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Button(
            modifier = Modifier.width(150.dp),
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primaryContainer),
            onClick = onClickPrevious
        ) {
            Text(
                text = stringResource(R.string.previous),
                color = MaterialTheme.colorScheme.onPrimaryContainer,
                textAlign = TextAlign.Center
            )
        }
        Button(
            modifier = Modifier.width(150.dp),
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primaryContainer),
            onClick = onClickNext
        ) {
            Text(
                text = stringResource(R.string.next),
                color = MaterialTheme.colorScheme.onPrimaryContainer,
                textAlign = TextAlign.Center
            )
        }
    }
}


@Preview(showBackground = true, showSystemUi = true)
@Composable
fun GreetingPreview() {
    ArtSpaceTheme {
        ArtSpaceApp()
    }
}