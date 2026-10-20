# VectorSmart v1.1 · Smart AI Estúdio Vetorial (Beta)

## O que foi implementado

- Comando **File → Scripts → Smart AI · Estúdio Vetorial…**.
- Criação de documento SVG editável a partir de uma descrição em português e parâmetros de composição.
- Título e subtítulo editáveis, SVG com elementos identificados, formas vetoriais, quatro estilos (Geométrico, Minimalista, Gradiente e Selo vetorial), quatro paletas e uma cor de destaque.
- Formatos 1080×1440, 1080×1080, 1080×1920 e 1024×1024.
- Execução local sem internet, tokens ou credenciais.
- Escape de texto SVG, limite de entrada e validação da cor hexadecimal.
- Código-fonte anterior de Smart Social, Smart QR Pro e executor JSX preservado.
- Workflows existentes de MSI/Windows e Linux preservados.

## Limite importante

Este **Smart AI Beta** é um assistente determinístico de composição. A descrição identifica o objetivo da arte, mas ainda **não** é interpretada por um modelo generativo de linguagem/imagem. O desenho depende dos estilos, textos e opções escolhidos. Ele não faz edição semântica de vetores já existentes e não gera ilustrações inéditas a partir de uma descrição livre. Para isso, é necessária uma segunda etapa com um modelo de IA adequado, autenticação de provedor e conversão validada de resultados em paths SVG.

## Como usar

1. File → Scripts → Smart AI · Estúdio Vetorial…
2. Digite uma descrição, título e subtítulo.
3. Escolha formato, composição, paleta e destaque.
4. Clique em **Criar SVG editável**.
5. Edite os elementos no VectorSmart.

## Compilação

Faça push para GitHub e acompanhe os workflows em `.github/workflows/`. O pacote é código-fonte, não contém instaladores compilados. Não foi possível executar `cargo check` neste ambiente, porque Rust/Cargo não estavam instalados.
