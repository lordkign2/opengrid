package com.opengrid.android.ffi

import java.nio.ByteBuffer

/**
 * Manual JNI bridge for OpenGrid (Diplomat-style)
 */
object DiplomatBridge {
    static {
        System.loadLibrary("opengrid_ffi")
    }

    // Engine
    @JvmStatic
    external fun OpenGridEngine_new(): Long
    
    @JvmStatic
    external fun OpenGridEngine_create_node(enginePtr: Long, name: String): Long
    
    @JvmStatic
    external fun OpenGridEngine_destroy(enginePtr: Long)

    // NodeHandle
    @JvmStatic
    external fun NodeHandle_new_ephemeral(): Long
    
    @JvmStatic
    external fun NodeHandle_submit_event(nodePtr: Long, payload: ByteArray): Long
    
    @JvmStatic
    external fun NodeHandle_current_version(nodePtr: Long): Long
    
    @JvmStatic
    external fun NodeHandle_destroy(nodePtr: Long)
}

/**
 * High-level wrapper for OpenGridEngine
 */
class OpenGridEngine private constructor(internal val ptr: Long) {
    companion object {
        fun create(): OpenGridEngine {
            val ptr = DiplomatBridge.OpenGridEngine_new()
            return OpenGridEngine(ptr)
        }
    }

    fun createNode(name: String): NodeHandle {
        val nodePtr = DiplomatBridge.OpenGridEngine_create_node(ptr, name)
        return NodeHandle(nodePtr)
    }

    protected fun finalize() {
        DiplomatBridge.OpenGridEngine_destroy(ptr)
    }
}

/**
 * High-level wrapper for NodeHandle
 */
class NodeHandle internal constructor(internal val ptr: Long) {
    companion object {
        fun newEphemeral(): NodeHandle {
            val ptr = DiplomatBridge.NodeHandle_new_ephemeral()
            return NodeHandle(ptr)
        }
    }

    fun submitEvent(payload: ByteArray): Long {
        return DiplomatBridge.NodeHandle_submit_event(ptr, payload)
    }

    fun currentVersion(): Long {
        return DiplomatBridge.NodeHandle_current_version(ptr)
    }

    protected fun finalize() {
        DiplomatBridge.NodeHandle_destroy(ptr)
    }
}
