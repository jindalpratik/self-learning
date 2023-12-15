package com.example.businesscard

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Call
import androidx.compose.material.icons.rounded.Email
import androidx.compose.material.icons.rounded.Share
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.businesscard.ui.theme.BusinessCardTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            BusinessCardTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    BusinessCardApp()
                }
            }
        }
    }
}

@Composable
fun BusinessCardApp() {
    Column(modifier = Modifier
        .fillMaxSize()
        .background(color = Color(0xFFD2E8D4))
    ) {
        PersonalInfoScreen(
            displayImage = painterResource(R.drawable.android_logo),
            fullName = stringResource(R.string.full_name),
            title = stringResource(R.string.title),
            modifier = Modifier.weight(4f)
        )
        ContactInfoScreen(
            contactNumber = stringResource(R.string.contact_number),
            socialLink = stringResource(R.string.social_link),
            email = stringResource(R.string.email_address),
            modifier = Modifier.weight(1f)
        )
    }
}

@Composable
private fun PersonalInfoScreen(
    displayImage: Painter,
    fullName: String,
    title: String,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Image(
            painter = displayImage,
            contentDescription = null,
            contentScale = ContentScale.Fit,
            modifier = Modifier
                .size(120.dp)
                .background(color = Color(0xFF073042))
        )
        Text(
            text = fullName,
            fontSize = 50.sp,
            fontWeight = FontWeight.Light,
            color = Color.Black
        )
        Text(
            text = title,
            color = Color(0xFF267F52),
            fontWeight = FontWeight.Bold
        )
    }
}

@Composable
private fun ContactInfoScreen(
    contactNumber: String,
    socialLink: String,
    email: String,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(bottom = 40.dp),
        verticalArrangement = Arrangement.Bottom ,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Column(
            modifier = Modifier.fillMaxHeight(),
            verticalArrangement = Arrangement.SpaceEvenly
        ) {
            Row(
                modifier = Modifier
            ) {
                Icon(
                    Icons.Rounded.Call,
                    tint = Color(0xFF267F52),
                    contentDescription = null,
                    modifier = Modifier.padding(end = 15.dp)
                )
                Text(text = contactNumber, color = Color.Black)
            }
            Row(
                modifier = Modifier
            ) {
                Icon(
                    Icons.Rounded.Share,
                    tint = Color(0xFF267F52),
                    contentDescription = null,
                    modifier = Modifier.padding(end = 15.dp)
                )
                Text(text = socialLink, color = Color.Black)
            }
            Row(
                modifier = Modifier
            ) {
                Icon(
                    Icons.Rounded.Email,
                    tint = Color(0xFF267F52),
                    contentDescription = null,
                    modifier = Modifier.padding(end = 15.dp)
                )
                Text(text = email, color = Color.Black)
            }
        }
    }
}

@Preview(showBackground = true, showSystemUi = true)
@Composable
fun BusinessCardAppPreview() {
    BusinessCardTheme {
        BusinessCardApp()
    }
}