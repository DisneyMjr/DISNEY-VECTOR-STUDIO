# VectorSmart v1.0 · Smart Social (versão inicial)

**Menu:** File → Scripts → Smart Social · Artes em lote…

## O que está implementado

- Criação em lote de SVG vetoriais editáveis a partir de CSV colado no diálogo.
- Formatos: Post 1080×1440, Quadrado 1080×1080, Story 1080×1920.
- Colunas CSV: `titulo` (obrigatória), `subtitulo`, `chamada`, `cor_fundo`, `cor_texto`.
- Cores hexadecimais por registro, quebra automática básica de texto, escape de XML, suporte a acentos UTF-8.
- Pré-validação de campos e limite configurável de 1 a 200 artes.
- Cada linha vira um documento SVG separado no editor, que pode ser editado e exportado.
- Mantido Smart QR Pro, executor JSX e workflows Windows/Linux do código existente.

## Exemplo

```csv
titulo,subtitulo,chamada,cor_fundo,cor_texto
Promoção especial,Novidades da semana,Saiba mais,#14213D,#FFFFFF
Novo produto,Conheça nossa linha,Peça seu orçamento,#683ACF,#FFFFFF
```

## Limitações / próximos passos

- A primeira versão usa CSV colado, não importa XLSX diretamente.
- Ainda não há fotos, logos, camadas de modelo, layout automático sofisticado, exportação automática em pasta ou geração de carrossel.
- O texto pode ultrapassar a área em títulos muito longos. Inspecione as artes antes de publicar.
- Testar a compilação Windows/Linux no GitHub Actions antes de distribuir.
- Os pacotes MSI, DEB, RPM e AppImage não estão compilados neste ZIP; são gerados pelos workflows.
