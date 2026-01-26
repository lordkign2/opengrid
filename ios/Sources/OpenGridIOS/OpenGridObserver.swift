import Foundation
import Combine

// Import the UniFFI-generated bindings
import OpenGridFFI

/**
 * ObservableObject that interfaces with the Rust OpenGrid engine
 *
 * This class enforces the architectural boundary that all state and business logic
 * resides in the Rust engine. The UI layer only sends commands and receives snapshots.
 */
class OpenGridObserver: ObservableObject {
    
    @Published var engineStatus: String = "Not initialized"
    
    // Handles for the Rust engine and nodes
    private var engineHandle: Any?
    private var nodeHandle: NodeHandleWrapper?
    
    init() {
        // Initialize with default state
    }
    
    /**
     * Initialize the Rust engine
     */
    func initializeEngine() {
        DispatchQueue.global(qos: .background).async {
            self.updateStatus("Initializing engine...")
            
            // Initialize the Rust engine
            do {
                self.engineHandle = create_engine()
                
                // Create a dummy node
                self.createDummyNode()
                
                DispatchQueue.main.async {
                    self.updateStatus("Engine initialized successfully")
                }
            } catch {
                DispatchQueue.main.async {
                    self.updateStatus("Engine init failed: \(error.localizedDescription)")
                }
            }
        }
    }
    
    /**
     * Create a dummy node for testing purposes
     */
    private func createDummyNode() {
        guard let engineHandle = self.engineHandle else {
            updateStatus("Cannot create node: engine not initialized")
            return
        }
        
        let config = NodeConfigWrapper(
            name: "iOS Node",
            description: "Node running on iOS device",
            metadata: ["platform": "ios", "timestamp": String(Date().timeIntervalSince1970)]
        )
        
        do {
            self.nodeHandle = try create_node(engine: engineHandle, config: config)
        } catch {
            updateStatus("Failed to create node: \(error.localizedDescription)")
        }
    }
    
    /**
     * Submit a dummy event to the engine
     */
    func submitDummyEvent() {
        DispatchQueue.global(qos: .background).async {
            self.updateStatus("Submitting dummy event...")
            
            do {
                guard let nodeHandle = self.nodeHandle else {
                    throw NSError(domain: "OpenGrid", code: 1, userInfo: [NSLocalizedDescriptionKey: "Node not initialized"])
                }
                
                let eventData = "dummy_event_\(Date().timeIntervalSince1970)".data(using: .utf8)!
                try submit_event(node_handle: nodeHandle, event_data: eventData)
                
                DispatchQueue.main.async {
                    self.updateStatus("Dummy event submitted")
                    
                    // Get a state snapshot
                    self.getStateSnapshot()
                }
            } catch {
                DispatchQueue.main.async {
                    self.updateStatus("Event submission failed: \(error.localizedDescription)")
                }
            }
        }
    }
    
    /**
     * Get a state snapshot from the engine
     */
    private func getStateSnapshot() {
        DispatchQueue.global(qos: .background).async {
            do {
                guard let nodeHandle = self.nodeHandle else {
                    throw NSError(domain: "OpenGrid", code: 1, userInfo: [NSLocalizedDescriptionKey: "Node not initialized"])
                }
                
                let _ = try get_state_snapshot(node_handle: nodeHandle)
                
                DispatchQueue.main.async {
                    self.updateStatus("State snapshot retrieved")
                }
            } catch {
                DispatchQueue.main.async {
                    self.updateStatus("Snapshot retrieval failed: \(error.localizedDescription)")
                }
            }
        }
    }
    
    private func updateStatus(_ status: String) {
        DispatchQueue.main.async {
            self.engineStatus = status
        }
    }
}