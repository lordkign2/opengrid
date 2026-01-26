package com.opengrid.android.ui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.opengrid.android.theme.OpenGridAndroidTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialize the ViewModel that connects to the Rust engine
        val viewModel = OpenGridViewModel(this.application)
        
        setContent {
            OpenGridAndroidTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    OpenGridScreen(viewModel = viewModel)
                }
            }
        }
    }
}

@Composable
fun OpenGridScreen(viewModel: OpenGridViewModel) {
    val engineStatus by viewModel.engineStatus.observeAsState("Initializing...")
    
    OpenGridContent(
        engineStatus = engineStatus,
        onInitializeClick = { viewModel.initializeEngine() },
        onSubmitEventClick = { viewModel.submitDummyEvent() }
    )
}

@Composable
fun OpenGridContent(
    engineStatus: String,
    onInitializeClick: () -> Unit,
    onSubmitEventClick: () -> Unit
) {
    androidx.compose.foundation.layout.Column(
        modifier = androidx.compose.foundation.layout.padding(16.dp)
    ) {
        Text(text = "OpenGrid Status: $engineStatus")
        androidx.compose.material3.Button(onClick = onInitializeClick) {
            Text("Initialize Engine")
        }
        androidx.compose.material3.Button(onClick = onSubmitEventClick) {
            Text("Submit Dummy Event")
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    OpenGridAndroidTheme {
        OpenGridContent(
            engineStatus = "Ready",
            onInitializeClick = {},
            onSubmitEventClick = {}
        )
    }
}