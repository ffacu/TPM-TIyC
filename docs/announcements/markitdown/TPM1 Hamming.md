TEORÍA DE LA INFORMACIÓN Y LA COMUNICACION
LABORATORIO  Nº1 - AÑO 2026: PROTECCION DE ARCHIVOS

Realizar un programa de manera que dado un archivo de tipo texto  (.txt),  de la posibilidad de:

-  Proteger el archivo aplicando Hamming (módulos de 8 bits, 1024 bits, 16384 bits),
    Cada módulo está integrado por los bits de información y los bits de control.

-  Hacer una función que pueda introducir errores… máximo uno por módulo.
O  sea,  para  cada  uno  de  los  módulos  con  una  función  aleatoria  decidir  si  en  ese  módulo  se
introduce error o no, y en caso de que si, con otra función aleatoria definir en que posición de ese
módulo se introducirá error.

-  Dar  la  posibilidad  de  decodificar  archivo  generado  y  habiéndole  introducido  errores,  hacer
proceso inverso decodificando y obteniendo la información original.

- Dar la opción también de poder ver archivo con errores introducidos, es decir decodificarlo
co errores y en modo ascii texto veremos errores con respecto al archivo texto original.

-Trabajar  a  nivel  de  bits  (trabajar  con  enmascaramiento)  y  buscar  la  eficiencia  de  espacio para
ver resultados reales en el tamaño del archivo. (peso en bytes)

- Debe poder visualizarse texto original y texto recuperado en pantalla para ver diferencias.
-----------------------------------------------------------------------------------------------------------------------

El programa deberá tener un MENU DE OPCIONES que  permita:

-CARGAR UN ARCHIVO (.TXT)

-PROTEGER  ARCHIVO  (.TXT)  GENERANDO  ARCHIVO  PROTEGIDO  CON  BLOQUE  A
ELECCION DE

         8 bits   guardado con la extensión “.HA1”
   1024 bits  guardado con la extensión  “.HA2”
 16384 bits guardado con la extensión   “.HA3”

-INTRODUCIR ERRORES  (.HAx)    dde x=1,2,3
(HEx= Hamming con Error  sobre Archivo x)

  GENERANDO  “.HEx”

-DESPROTEGER ARCHIVO SIN CORREGIR (.HAx, .HEx)          GENERANDO  “.DEx”
(DEx= Archivo Decodificado con Error sobre Archivo x)    dde x=1,2,3

-DESPROTEGER ARCHIVO CORRIGIENDO (.HAx, .HEx)          GENERANDO  “.DCx”
(DEx= Archivo Decodificado Corregido sobre Archivo x)      dde x=1,2,3

-ENCRIPTAR ARCHIVO ENVIADO CON DIA Y HORA DE APERTURA

-VER ARCHIVOS EN PANTALLA
(QUE MUESTRE 2 ARCHIVOS, Y SE PUEDA HACER SCROLLING DE LOS MISMOS)
(no es obligatorio por ahora)
(MARCAR EN ARCHIVO CON ERRORES LOS CARACTERES DONDE SE PRODUCE ERROR CON COLOR ROJO DE LETRA)
(no es obligatorio por ahora)

CUALQUIER DUDA COMUNICARSE A marioasilvestri@gmail.com
TENER EN CUENTA QUIENES USEN LENGUAJE C:
abrir los archivos con  fopen( "<nombre del archivo>.txt", "rb")   ¡!!!

