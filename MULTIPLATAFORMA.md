# VectorSmart multiplataforma: estado real

## Windows / Linux
Os workflows existentes continuam sem alteracoes. A compilacao e os instaladores dependem da validacao no GitHub Actions.

## macOS Intel e Apple Silicon
Adicionado `.github/workflows/vectorsmart-macos.yml` para compilar o projeto Rust existente e executar `packaging/macos/package.sh --arch universal`, produzindo um DMG e zip de CLI como artefatos. O empacotador macOS ja existia no projeto e requer fontes externas licenciadas. Assinatura ad hoc sem certificados nao equivale a notarizacao; o usuario pode precisar permitir manualmente a execucao.

## iPhone e iPad (iOS / iPadOS)
`mobile/VectorSmartIOS/` contem um **prototipo nativo SwiftUI**, separado do editor Rust completo. Inclui documento local JSON, objetos retangulo/elipse/texto editaveis, arrastar, alterar tamanho/cor, zoom e exportacao SVG via Share Sheet. Nao possui compatibilidade de documentos com o projeto desktop, curvas Bezier, varias ferramentas, sistema de camadas, Smart AI, Smart Social, QR Pro ou Canva, nem a mesma renderizacao do motor Rust.

**Nao e um port do editor desktop completo.** Para obter paridade e compartilhamento de arquivos, sera necessario adaptar o motor Rust em uma biblioteca `staticlib` para targets `aarch64-apple-ios` e `aarch64-apple-ios-sim`, criar camada FFI/UniFFI, reimplementar integracoes e desenvolver interface multitouch e Apple Pencil. Isso exige testes com Xcode em macOS e aparelhos/simuladores iOS reais.

### Como abrir o prototipo iOS
1. No macOS, instale Xcode e crie um projeto iOS App SwiftUI com Swift e iOS 17+.
2. Remova o arquivo App criado pelo template para evitar dois pontos de entrada `@main`.
3. Adicione os tres arquivos `.swift` de `mobile/VectorSmartIOS` ao target da aplicacao.
4. Registre o documento importado/exportado (`com.disneymkt.vectorsmart.document`) nas configuracoes de tipos de documento / UTExportedTypeDeclarations, e ajuste bundle ID e Signing & Capabilities.
5. Compile no simulador; confirme exportacao SVG em um segundo visualizador.

**Importante:** os arquivos sao fonte de um prototipo, nao um projeto Xcode `.xcodeproj` pronto; nao foram compilados nem testados em simulador neste ambiente.

## Validacao e publicacao
`cargo`/`rustc` e Xcode nao estao disponiveis neste ambiente. A integridade do ZIP e a estrutura dos arquivos foram verificadas, mas nao houve build real de Windows, Linux, macOS ou iOS. Publicar iOS via TestFlight/App Store requer Apple Developer e processo de assinatura/distribuicao.

## Licencas
As licencas e atribuicoes dos componentes de terceiros devem ser mantidas nos arquivos adequados, mesmo que a interface exiba somente a marca VectorSmart.
