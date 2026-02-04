package com.opengrid.android.ui

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.util.*

import com.opengrid.android.ffi.OpenGridEngine
import com.opengrid.android.ffi.NodeHandle

/**
 * ViewModel that interfaces with the Rust OpenGrid engine
 *
 * This ViewModel enforces the architectural boundary that all state and business logic
 * resides in the Rust engine. The UI layer only sends commands and receives snapshots.
 */
class OpenGridViewModel(application: Application) : AndroidViewModel(application) {
    
    private val _engineStatus = MutableLiveData<String>("Not initialized")
    val engineStatus: LiveData<String> = _engineStatus
    
    // Handles for the Rust engine and nodes
    private var engine: OpenGridEngine? = null
    private var node: NodeHandle? = null
    
    /**
     * Initialize the Rust engine
     */
    fun initializeEngine() {
        viewModelScope.launch {
            _engineStatus.value = "Initializing engine..."
            
            try {
                withContext(Dispatchers.IO) {
                    // Initialize the Rust engine via Diplomat bridge
                    engine = OpenGridEngine.create()
                    
                    // Create an initial node
                    engine?.let {
                        node = it.createNode("Android Node")
                    }
                }
                
                _engineStatus.value = "Engine initialized successfully"
            } catch (e: Exception) {
                _engineStatus.value = "Engine init failed: ${e.message}"
            }
        }
    }
    
    /**
     * Submit a dummy event to the engine
     */
    fun submitDummyEvent() {
        viewModelScope.launch {
            _engineStatus.value = "Submitting dummy event..."
            
            try {
                withContext(Dispatchers.IO) {
                    node?.let {
                        val eventData = "dummy_event_${Date().time}".toByteArray()
                        val version = it.submitEvent(eventData)
                        
                        if (version == 0L) {
                             throw Exception("Event submission returned version 0")
                        }
                    } ?: throw Exception("Node not initialized")
                }
                
                _engineStatus.value = "Dummy event submitted. Version: ${node?.currentVersion()}"
            } catch (e: Exception) {
                _engineStatus.value = "Event submission failed: ${e.message}"
            }
        }
    }
}