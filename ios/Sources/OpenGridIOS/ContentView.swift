import SwiftUI

struct ContentView: View {
    @ObservedObject var observer: OpenGridObserver
    
    var body: some View {
        VStack(spacing: 20) {
            Text("OpenGrid Status: \(observer.engineStatus)")
                .padding()
                .multilineTextAlignment(.center)
            
            Button(action: {
                observer.initializeEngine()
            }) {
                Text("Initialize Engine")
                    .padding()
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(8)
            }
            
            Button(action: {
                observer.submitDummyEvent()
            }) {
                Text("Submit Dummy Event")
                    .padding()
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(8)
            }
            
            Spacer()
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(observer: OpenGridObserver())
    }
}