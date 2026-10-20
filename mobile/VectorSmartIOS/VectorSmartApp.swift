import SwiftUI

@main
struct VectorSmartMobileApp: App {
    var body: some Scene {
        DocumentGroup(newDocument: VectorDocument()) { file in
            EditorView(document: file.$document)
        }
    }
}
