import rasterio
from rasterio.windows import Window

with rasterio.open("brazil_coverage-col11_2025.tif") as src:
    lat, lon = -4.626448, -63.670118

    # Converte lat/lon para linha e coluna no raster
    row, col = src.index(lon, lat)

    # Lê a janela 30x30 de uma só vez
    window = Window(col, row, 30, 30)
    data = src.read(1, window=window)

    # Imprime a matriz formatada
    for linha in data:
        for valor in linha:
            print(f"{valor:4}", end=" ")
        print()
