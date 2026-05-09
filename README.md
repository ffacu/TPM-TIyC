### Requisitos Previos (dependencias)

Necesitas tener instaladas las siguientes herramientas:
*   [Node.js](https://nodejs.org/) (Versión 18 o superior)
*   [pnpm](https://pnpm.io/es/installation) (`npm install -g pnpm`)
*   [Rust y Cargo](https://www.rust-lang.org/tools/install)
*   **Dependencias de Tauri:** Dependiendo el sistema operativo (Windows, Linux, macOS), Tauri requiere algunas librerías adicionales del sistema. Recomendamos leer la [Guía de Prerrequisitos de Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/).

### desde la carpeta tpm-tiyc ejecuta:

* pnpm install  // Dependencias del backend
* pnpm tauri dev  // Ejecuta el programa

# Manual de Usuario - Compactador de Archivos (Huffman)

A continuación, te explicamos cómo utilizar cada una de las funcionalidades disponibles.

## Menú de Opciones

### 1. Cargar Archivo de Texto
Para empezar, seleccioná el archivo que deseás procesar. Podés cargar solo archivos `.txt` (momentaneamente).

### 2. Compactar Archivo
Una vez cargado tu archivo, utilizá esta opción para comprimirlo. El sistema aplicará el método de Huffman y generará un nuevo archivo con la extensión `.huf`. Vas a notar que podés elegir entre compactar por caracteres o por palabras.

### 3. Descompactar Archivo
Si tenés un archivo previamente compactado (con extensión `.huf`), podés seleccionarlo y descompactarlo. El sistema te devolverá un archivo recuperado con la extensión `.dhu`, que contendrá exactamente la misma información que el original.

### 4. Ver Archivos en Pantalla
El programa cuenta con un visor que te permite visualizar el archivo original y el archivo obtenido en la misma pantalla. Vas a poder hacer *scrolling* de los mismos para comparar y revisar que la información se haya mantenido intacta tras el proceso.

### 5. Ver Estadísticas
Esta sección te ofrece una muestra estadística, en la que vas a poder ver y comparar los tamaños y pesos de los archivos: el original, el compactado y el descompactado. Esto te permitirá evaluar la eficiencia real de la compresión.

---

## Análisis y Recomendaciones de Uso

A partir de las distintas ejecuciones y pruebas realizadas con el sistema, preparamos un pequeño informe sobre los resultados obtenidos para que le saques el mayor provecho al programa:

- **Archivos Grandes:** El programa tiene un mejor rendimiento (mayor porcentaje de compresión) cuando compactás **archivos pesados y seleccionás la compresión por palabras**. Dado que las palabras tienden a repetirse con frecuencia en textos extensos, el árbol de Huffman logra optimizar mucho mejor el espacio asignando códigos más cortos a palabras enteras.
- **Archivos Livianos:** Cuando trabajes con archivos de menor tamaño, te conviene utilizar la **compresión por caracteres**.
- **Punto de Inflexión (Archivos muy pequeños):** Hay que tener en consideracion que existe un punto de peso mínimo para los archivos. Si el archivo original es *suficientemente liviano* (es decir, contiene muy poco texto), el archivo resultante compactado puede terminar siendo **más grande** que el original. Esto ocurre porque el archivo `.huf` debe guardar internamente la tabla de frecuencias (o el árbol de Huffman) necesaria para la posterior descompresión. En archivos muy chicos, el peso de esta metadata supera el ahorro de compresión del texto en sí.
