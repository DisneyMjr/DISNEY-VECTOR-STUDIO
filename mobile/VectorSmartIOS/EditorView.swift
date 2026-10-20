import SwiftUI
import UniformTypeIdentifiers

struct EditorView: View {
    @Binding var document: VectorDocument
    @State private var selected: UUID?
    @State private var zoom: CGFloat = 1
    @State private var dragOrigins: [UUID: CGPoint] = [:]

    private var svgFile: SVGFile { SVGFile(text: document.svg()) }

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                ScrollView([.horizontal, .vertical]) {
                    ZStack(alignment: .topLeading) {
                        Rectangle().fill(Color.white)
                        ForEach(document.shapes) { shape in
                            shapeView(shape)
                                .frame(width: shape.width, height: shape.height)
                                .position(x: shape.x + shape.width / 2, y: shape.y + shape.height / 2)
                                .overlay(alignment: .topLeading) { EmptyView() }
                                .onTapGesture { selected = shape.id }
                                .gesture(DragGesture(minimumDistance: 1)
                                    .onChanged { value in
                                        guard let idx = document.shapes.firstIndex(where: { $0.id == shape.id }) else { return }
                                        let initial = dragOrigins[shape.id] ?? CGPoint(x: shape.x, y: shape.y)
                                        if dragOrigins[shape.id] == nil { dragOrigins[shape.id] = initial }
                                        document.shapes[idx].x = max(0, initial.x + value.translation.width / zoom)
                                        document.shapes[idx].y = max(0, initial.y + value.translation.height / zoom)
                                        selected = shape.id
                                    }
                                    .onEnded { _ in dragOrigins.removeValue(forKey: shape.id) })
                        }
                    }
                    .frame(width: document.canvasWidth, height: document.canvasHeight)
                    .scaleEffect(zoom, anchor: .topLeading)
                    .frame(width: document.canvasWidth * zoom, height: document.canvasHeight * zoom, alignment: .topLeading)
                    .shadow(radius: 8)
                    .padding(24)
                }
                .background(Color(white: 0.91))
                HStack {
                    Button { add(.rectangle) } label: { Label("Retângulo", systemImage: "square") }
                    Button { add(.ellipse) } label: { Label("Elipse", systemImage: "circle") }
                    Button { add(.text) } label: { Label("Texto", systemImage: "textformat") }
                }
                .buttonStyle(.bordered)
                .padding(8)
                if let selection = document.shapes.firstIndex(where: { $0.id == selected }) {
                    Form {
                        TextField("Cor hexadecimal", text: $document.shapes[selection].colorHex)
                        if document.shapes[selection].kind == .text {
                            TextField("Texto", text: $document.shapes[selection].text)
                        }
                        Stepper("Largura: \(Int(document.shapes[selection].width))", value: $document.shapes[selection].width, in: 10...1000, step: 10)
                        Stepper("Altura: \(Int(document.shapes[selection].height))", value: $document.shapes[selection].height, in: 10...1000, step: 10)
                        Button("Excluir selecionado", role: .destructive) {
                            document.shapes.remove(at: selection)
                            selected = nil
                        }
                    }
                    .frame(height: 220)
                }
            }
            .navigationTitle("VectorSmart")
            .toolbar {
                ToolbarItemGroup(placement: .topBarTrailing) {
                    Button { zoom = max(0.15, zoom / 1.25) } label: { Image(systemName: "minus.magnifyingglass") }
                    Button { zoom = min(2, zoom * 1.25) } label: { Image(systemName: "plus.magnifyingglass") }
                    ShareLink(item: svgFile, preview: SharePreview("Arte vetorial SVG")) {
                        Image(systemName: "square.and.arrow.up")
                    }
                }
            }
        }
    }

    @ViewBuilder private func shapeView(_ shape: VectorShape) -> some View {
        let fill = Color(hex: shape.colorHex)
        switch shape.kind {
        case .rectangle: Rectangle().fill(fill)
        case .ellipse: Ellipse().fill(fill)
        case .text:
            Text(shape.text).font(.system(size: 44)).foregroundStyle(fill)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        }
    }

    private func add(_ kind: VectorShape.Kind) {
        var item = VectorShape(kind: kind)
        if kind == .text { item.width = 460; item.height = 80 }
        document.shapes.append(item)
        selected = item.id
    }
}

private extension Color {
    init(hex: String) {
        let value = hex.trimmingCharacters(in: CharacterSet(charactersIn: "#"))
        guard value.count == 6, let n = UInt64(value, radix: 16) else { self = .blue; return }
        self.init(red: Double((n >> 16) & 255) / 255, green: Double((n >> 8) & 255) / 255, blue: Double(n & 255) / 255)
    }
}

struct SVGFile: Transferable {
    var text: String
    static var transferRepresentation: some TransferRepresentation {
        DataRepresentation(exportedContentType: .svg) { file in
            Data(file.text.utf8)
        }
        .suggestedFileName("VectorSmart.svg")
    }
}
