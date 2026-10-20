import SwiftUI
import UniformTypeIdentifiers

extension UTType {
    static let vectorSmartDocument = UTType(exportedAs: "com.disneymkt.vectorsmart.document", conformingTo: .json)
}

struct VectorShape: Codable, Identifiable, Equatable {
    enum Kind: String, Codable, CaseIterable { case rectangle, ellipse, text }
    var id: UUID = UUID()
    var kind: Kind
    var x: Double = 120
    var y: Double = 140
    var width: Double = 200
    var height: Double = 100
    var text: String = "DisneyMkt"
    var colorHex: String = "#2563EB"
}

struct VectorDocument: FileDocument {
    static var readableContentTypes: [UTType] { [.vectorSmartDocument] }
    static var writableContentTypes: [UTType] { [.vectorSmartDocument] }

    var shapes: [VectorShape] = [VectorShape(kind: .rectangle)]
    var canvasWidth: Double = 1080
    var canvasHeight: Double = 1440

    init() {}

    init(configuration: ReadConfiguration) throws {
        guard let data = configuration.file.regularFileContents else { throw CocoaError(.fileReadCorruptFile) }
        self = try JSONDecoder().decode(Self.self, from: data)
    }

    func fileWrapper(configuration: WriteConfiguration) throws -> FileWrapper {
        FileWrapper(regularFileWithContents: try JSONEncoder().encode(self))
    }

    func svg() -> String {
        func esc(_ raw: String) -> String {
            raw.replacingOccurrences(of: "&", with: "&amp;")
               .replacingOccurrences(of: "<", with: "&lt;")
               .replacingOccurrences(of: ">", with: "&gt;")
               .replacingOccurrences(of: "\"", with: "&quot;")
        }
        var parts: [String] = []
        for s in shapes {
            let color = s.colorHex.range(of: "^#[0-9a-fA-F]{6}$", options: .regularExpression) != nil ? s.colorHex : "#2563EB"
            switch s.kind {
            case .rectangle:
                parts.append("<rect x=\"\(s.x)\" y=\"\(s.y)\" width=\"\(max(s.width, 1))\" height=\"\(max(s.height, 1))\" fill=\"\(color)\"/>")
            case .ellipse:
                parts.append("<ellipse cx=\"\(s.x + s.width / 2)\" cy=\"\(s.y + s.height / 2)\" rx=\"\(max(s.width / 2, 1))\" ry=\"\(max(s.height / 2, 1))\" fill=\"\(color)\"/>")
            case .text:
                parts.append("<text x=\"\(s.x)\" y=\"\(s.y + 44)\" font-size=\"44\" font-family=\"sans-serif\" fill=\"\(color)\">\(esc(s.text))</text>")
            }
        }
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"\(canvasWidth)\" height=\"\(canvasHeight)\" viewBox=\"0 0 \(canvasWidth) \(canvasHeight)\">\n" + parts.joined(separator: "\n") + "\n</svg>"
    }
}

extension VectorDocument: Codable {}
