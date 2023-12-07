package com.example.artspace

import android.os.Bundle
import android.widget.ImageView
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
import androidx.compose.ui.graphics.Color
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
    //todo: Make it completely based on the number so something like R.drawable.image_asset_$imageAssetBase
    when(imageAssetBase) {
        1 -> {
            imageAssetId = painterResource(id = R.drawable.image_asset_1)
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
                modifier = Modifier
                    .fillMaxWidth()
            )
            Spacer(modifier = Modifier.height(20.dp))
            ArtSpaceChangeArtworkRow(
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
    modifier: Modifier = Modifier
) {
    Image(
        painter = imageAssetId,
        contentDescription = stringResource(R.string.image_asset_1_title),
        modifier = modifier.padding(all = 30.dp)
    )
}

@Composable
fun ArtSpaceArtworkDescription(modifier: Modifier = Modifier) {
    Column(
        modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Column(
            modifier = Modifier
                .background(color = Color(0xffecebf4))
                .padding(20.dp)
        ) {
            Text(
                text = stringResource(R.string.image_asset_1_title),
                fontSize = 25.sp,
                fontWeight = FontWeight.Light
            )
            Row {
                Text(
                    text = stringResource(R.string.image_asset_1_artist),
                    fontWeight = FontWeight.Bold
                )
                Spacer(modifier = Modifier.width(5.dp))
                Text(
                    text = stringResource(R.string.image_asset_1_year),
                    fontWeight = FontWeight.Light
                )
            }
        }
    }
}

@Composable
fun ArtSpaceChangeArtworkRow(modifier: Modifier = Modifier) {
    Row(
        modifier = modifier.fillMaxSize(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Button(
            modifier = Modifier.width(150.dp),
            onClick = { /*TODO*/ }
        ) {
            Text(
                text = stringResource(R.string.previous),
                textAlign = TextAlign.Center
            )
        }
        Button(
            modifier = Modifier.width(150.dp),
            onClick = { /*TODO*/ }
        ) {
            Text(
                text = stringResource(R.string.next),
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