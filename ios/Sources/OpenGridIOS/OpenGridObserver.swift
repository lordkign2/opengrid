import Foundation
import Combine

/**
 * ObservableObject that interfaces with the Rust OpenGrid engine
 *
 * This class enforces the architectural boundary that all state and business logic
 * resides in the Rust engine. The UI layer only sends commands and receives snapshots.
 */
class OpenGridObserver: ObservableObject {
    
    @Published var engineStatus: String = "Not initialized"
    
    // Handles for the Rust engine and nodes
    private var engine: OpenGridEngine?
    private var node: NodeHandle?
    
    init() {
        // Initialize with default state
    }
    
    /**
     * Initialize the Rust engine
     */
    func initializeEngine() {
        DispatchQueue.global(qos: .background).async {
            self.updateStatus("Initializing engine...")
            
            // Initialize the Rust engine via Diplomat bridge
            self.engine = OpenGridEngine()
            
            // Create an initial node
            if let engine = self.engine {
                self.node = engine.createNode(name: "iOS Node")
            }
            
            DispatchQueue.main.async {
                self.updateStatus("Engine initialized successfully")
            }
        }
    }
    
    /**
     * Submit a dummy event to the engine
     */
    func submitDummyEvent() {
        DispatchQueue.global(qos: .background).async {
            self.updateStatus("Submitting dummy event...")
            
            do {
                guard let node = self.node else {
                    throw NSError(domain: "OpenGrid", code: 1, userInfo: [NSLocalizedDescriptionKey: "Node not initialized"])
                }
                
                let eventData = "dummy_event_\(Date().timeIntervalSince1970)".data(using: .utf8)!
                let version = node.submitEvent(payload: eventData)
                
                if version == 0 {
                    throw NSError(domain: "OpenGrid", code: 2, userInfo: [NSLocalizedDescriptionKey: "Event submission failed"])
                }
                
                DispatchQueue.main.async {
                    self.updateStatus("Dummy event submitted. Version: \(node.currentVersion())")
                }
            } catch {
                DispatchQueue.main.async {
                    self.updateStatus("Event submission failed: \(error.localizedDescription)")
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