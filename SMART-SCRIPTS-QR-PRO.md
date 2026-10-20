# VectorSmart | Smart Scripts e Smart QR Pro (experimental)

## Recursos

Em **File > Scripts > Smart QR Pro**, preencha texto/URL, tamanho em polegadas, cor dos módulos (`#RRGGBB`), cor de fundo e margem (4 módulos recomendado). O editor executa o gerador vetorial integrado e abre o SVG resultante na prancheta.

Em **File > Scripts > Executar JSX (experimental)**, selecione um `.jsx` de origem confiável e confirme sua execução. O suporte a ExtendScript é parcial, destinado principalmente ao script QR.jsx testado. O Node.js 22 deve estar instalado e acessível pelo PATH. No Windows, o processo Node é chamado sem a janela de console.

**Atenção:** JavaScript/JSX não confiável representa risco. O `vm` do Node.js não é uma sandbox de segurança. Execute apenas scripts que você verificou. Formulários ScriptUI e a API Adobe Illustrator não são plenamente implementados. Os parâmetros de QR se aplicam ao executor compatível.

## Windows e Linux

Workflows mantidos do projeto-base:
- `.github/workflows/disney-vector-msi.yml` (MSI)
- `.github/workflows/vectorsmart-linux.yml` (pacotes Linux)
- `.github/workflows/smart-release.yml` (Release)

Os arquivos foram preservados, mas a compilação Rust e a geração dos instaladores ainda devem ser validadas no GitHub Actions.

## Teste independente

```sh
node tools/jsx-runner.cjs tools/smart-qr-base.jsx --data "https://instagram.com/disneymkt" --size 2 --foreground '#1E40AF' --background '#FFFFFF' --margin 4 --out teste.svg
```

Créditos e licenças do script original de David Street e Kazuhiko Arase estão mantidos em `tools/smart-qr-base.jsx`.
