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

import com.opengrid.android.ffi.*

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
    private var engineHandle: Any? = null
    private var nodeHandle: NodeHandleWrapper? = null
    
    /**
     * Initialize the Rust engine
     */
    fun initializeEngine() {
        viewModelScope.launch {
            _engineStatus.value = "Initializing engine..."
            
            try {
                withContext(Dispatchers.IO) {
                    // Initialize the Rust engine
                    engineHandle = create_engine()
                    
                    // Create a dummy node
                    createDummyNode()
                }
                
                _engineStatus.value = "Engine initialized successfully"
            } catch (e: Exception) {
                _engineStatus.value = "Engine init failed: ${e.message}"
            }
        }
    }
    
    /**
     * Create a dummy node for testing purposes
     */
    private fun createDummyNode() {
        if (engineHandle != null) {
            val config = NodeConfigWrapper(
                name = "Android Node",
                description = "Node running on Android device",
                metadata = mapOf(
                    "platform" to "android",
                    "timestamp" to System.currentTimeMillis().toString()
                )
            )
            
            val result = create_node(engineHandle!!, config)
            
            when (result) {
                is OpenGridResult.Ok -> {
                    nodeHandle = result.value
                }
                is OpenGridResult.Error -> {
                    _engineStatus.postValue("Failed to create node: ${result.message}")
                }
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
                    if (nodeHandle != null) {
                        val eventData = "dummy_event_${Date().time}".toByteArray()
                        val result = submit_event(nodeHandle!!, eventData)
                        
                        when (result) {
                            is OpenGridResult.Ok -> {
                                // Event submitted successfully
                            }
                            is OpenGridResult.Error -> {
                                throw Exception(result.message)
                            }
                        }
                    } else {
                        throw Exception("Node not initialized")
                    }
                }
                
                _engineStatus.value = "Dummy event submitted"
                
                // Get a state snapshot
                getStateSnapshot()
            } catch (e: Exception) {
                _engineStatus.value = "Event submission failed: ${e.message}"
            }
        }
    }
    
    /**
     * Get a state snapshot from the engine
     */
    private fun getStateSnapshot() {
        viewModelScope.launch {
            try {
                withContext(Dispatchers.IO) {
                    if (nodeHandle != null) {
                        val result = get_state_snapshot(nodeHandle!!)
                        
                        when (result) {
                            is OpenGridResult.Ok -> {
                                // Process the snapshot as needed
                                // For now, just verify we got a result
                            }
                            is OpenGridResult.Error -> {
                                throw Exception(result.message)
                            }
                        }
                    } else {
                        throw Exception("Node not initialized")
                    }
                }
                
                _engineStatus.value = "State snapshot retrieved"
            } catch (e: Exception) {
                _engineStatus.value = "Snapshot retrieval failed: ${e.message}"
            }
        }
    }
}