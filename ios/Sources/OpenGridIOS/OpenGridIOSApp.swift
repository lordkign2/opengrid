import SwiftUI

@main
struct OpenGridIOSApp: App {
    @StateObject private var observer = OpenGridObserver()
    
    var body: some Scene {
        WindowGroup {
            ContentView(observer: observer)
        }
    }
}