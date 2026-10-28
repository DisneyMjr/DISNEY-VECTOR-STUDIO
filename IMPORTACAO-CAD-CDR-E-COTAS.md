# VectorSmart DisneyMkt — CAD, CorelDRAW e medidas (primeira etapa)

## O que foi alterado no código-fonte

- **File → Importar DWG / CDR…**: abre um seletor para esses formatos.
- DWG: usa o conversor externo **LibreDWG `dwg2SVG`** e abre o SVG resultante como documento editável no VectorSmart.
- CDR: usa **LibreOffice Draw com suporte libcdr** em modo sem interface e abre a conversão SVG.
- **View → Cotas da seleção (mm)…**: mostra largura e altura da caixa envolvente dos objetos selecionados em milímetros. Não desenha ainda as linhas de cota sobre a prancheta.
- Arquivos DWG/CDR originais não são alterados.

## Dependências externas e limitações

O recurso precisa de `dwg2SVG` e/ou `libreoffice` instalados e disponíveis no PATH. Os binários não vêm no ZIP. A disponibilidade é diferente em Windows, Linux e macOS. A importação não é nativa, não garante fidelidade, nem funciona para todo tipo ou versão de DWG/CDR. Conversão em SVG pode perder blocos, camadas, cotas CAD, fontes, escalas, objetos 3D e outros recursos. Verifique o desenho antes de fabricar, cortar, construir ou imprimir.

O cálculo de cota assume pontos gráficos (72 pt = 1 polegada). A medida exibida é **de caixa envolvente**, não uma cota CAD associativa. Documentos CAD podem usar unidades de desenho arbitrárias, por isso é necessário conferir escala antes de confiar nos valores. Escalas 1:50, 1:100 e cotas associativas não foram implementadas.

## Distribuição e testes

Workflows de Windows, Linux e macOS do projeto foram preservados. Não executamos cargo check nem compilamos os instaladores neste ambiente, por falta de compilador Rust. É preciso validar o job GitHub Actions em cada plataforma e testar DWG/CDR reais.

## Próximos passos para cotagem completa

1. Cotas horizontais, verticais, alinhadas, angulares, raio e diâmetro como objetos vetoriais da prancheta.
2. Unidades mm, cm, m, pol e pt, escala e precisão configurável.
3. Ajustar cotas automaticamente ao mover/redimensionar objetos (associatividade).
4. Importação CAD mais fiel com leitura de DXF preservando unidades e layers, e avisos detalhados de perda na conversão.
