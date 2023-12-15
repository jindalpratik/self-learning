package com.example.lemonade

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.lemonade.ui.theme.LemonadeTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LemonadeTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    LemonApp()
                }
            }
        }
    }
}

@Composable
fun LemonApp() {

    var step by remember { mutableStateOf(1) }
    var squeeze by remember { mutableStateOf(0) }


    Text(
        modifier = Modifier
            .fillMaxWidth()
            .padding(20.dp),
        fontSize = 20.sp,
        fontWeight = FontWeight.Bold,
        textAlign = TextAlign.Center,
        text = stringResource(R.string.app_name)
    )
    when(step) {
        1 ->  {
            LemonImageAndText(
                drawableResourceId = R.drawable.lemon_tree,
                textResourceId = R.string.lemon_tree_text,
                descriptionResourceId = R.string.lemon_tree_description,
                onImageClick = {
                    step = 2
                    squeeze = (2..4).random()
                }
            )
        }
        2 -> {
            LemonImageAndText(
                drawableResourceId = R.drawable.lemon_squeeze,
                descriptionResourceId = R.string.lemon_description,
                textResourceId = R.string.lemon_text,
                onImageClick = {
                    squeeze--
                    if(squeeze == 0) {
                        step = 3
                    }
                }
            )
        }
        3 -> {
            LemonImageAndText(
                drawableResourceId = R.drawable.lemon_drink,
                descriptionResourceId = R.string.glass_of_lemonade_description,
                textResourceId = R.string.glass_of_lemonade_text,
                onImageClick = {
                    step = 4
                }
            )
        }
        4 -> {
            LemonImageAndText(
                drawableResourceId = R.drawable.lemon_restart,
                descriptionResourceId = R.string.empty_glass_description,
                textResourceId = R.string.empty_glass_text,
                onImageClick = {
                    step = 1
                }
            )
        }
    }


}

@Composable
fun LemonImageAndText(
    modifier: Modifier = Modifier,
    drawableResourceId: Int,
    descriptionResourceId: Int,
    textResourceId: Int,
    onImageClick: () -> Unit
) {
    Column(
        modifier = modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Button(
            modifier = Modifier,
            shape = RoundedCornerShape(20.dp),
            colors = ButtonDefaults.buttonColors(containerColor = Color(0xFFc3ecd2)),
            onClick = onImageClick
        ) {
            Image(
                painter = painterResource(drawableResourceId),
                contentDescription = stringResource(descriptionResourceId)
            )
        }
        Spacer(modifier = Modifier.height(16.dp))
        Text(text = stringResource(textResourceId))
    }
}

@Preview(showBackground = true, showSystemUi = true)
@Composable
fun GreetingPreview() {
    LemonadeTheme {
        LemonApp()
    }
}