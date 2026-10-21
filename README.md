# automaton

Simulador de autômatos celulares em Rust com interface gráfica feita com macroquad. O programa abre uma janela, carrega uma cena descrita em TOML e anima a evolução do autômato, com controles para pausar, acelerar e interagir com o tabuleiro.

Estão implementados três autômatos. O primeiro é o Conway's Game of Life, que aceita tabuleiros definidos em cenas TOML e permite alternar células manualmente ou sortear um bloco de células com o mouse. O segundo é uma simulação de propagação de incêndio florestal em que a vegetação é distribuída aleatoriamente em três perfis (realismo, risco e didático), com direção e velocidade do vento configuráveis e focos iniciais escolhidos na cena. O terceiro é uma variante dessa simulação que lê uma janela de um GeoTIFF do MapBiomas a partir de latitude e longitude, converte as classes de bioma em tipos de combustível e permite alternar a visualização entre bioma e combustível, além de exibir uma legenda.

As cenas ficam em `scenes/` e são arquivos TOML identificados pelos tipos `conway`, `sfb` e `isfb`. O FPS e o caminho da cena são passados por linha de comando.

## Como executar

Conway:

    cargo run --release -- -s scenes/glider.toml

Incêndio com vegetação aleatória:

    cargo run --release -- -s scenes/sfb.toml

Incêndio sobre uma janela do mapa do Brasil:

    cargo run --release -- -s scenes/brazil.toml

Sem o argumento de cena, o programa abre um Conway vazio de 80x40:

    cargo run --release

As opções disponíveis são `-f` ou `--fps` para definir os quadros por segundo (padrão 1) e `-s` ou `--scene` para indicar o arquivo da cena.

## Controles

Esc ou Q encerra o programa, e Enter ou Espaço inicia ou pausa a simulação. O botão esquerdo do mouse acende uma célula na simulação de incêndio ou alterna uma célula no Conway, enquanto o botão direito acende uma vizinhança 3x3 no incêndio ou sorteia células no Conway. Na simulação integrada com o MapBiomas, K mostra ou oculta a legenda e M alterna entre a visualização por bioma e por tipo de combustível.

## Requisitos

É necessário ter Rust 1.85 ou superior, já que o projeto usa edition 2024, e a biblioteca GDAL instalada para a leitura do GeoTIFF.

## Estrutura

A implementação dos autômatos fica em `src/automaton/`, com um módulo para o Conway e outro para cada variante da simulação de incêndio. O loop principal e o tratamento de entrada estão em `src/runner.rs`, o desenho do tabuleiro, do título e da legenda em `src/view.rs`, e a leitura dos argumentos em `src/config.rs`. A pasta `scenes/` reúne as cenas TOML e o GeoTIFF usado pela cena do Brasil.
