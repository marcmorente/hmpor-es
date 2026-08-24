# bilingue — generador de la edición bilingüe de HPMOR

Lee `hpmor-en.html` (inglés) y `hpmor.html` (español) del raíz del repositorio
y genera dos artefactos:

- `hpmor-bilingue.html` — archivo único autocontenido: el libro en inglés con
  un asterisco gris al final de cada párrafo; al pulsarlo se abre un pop-up
  (Popover API, sin JavaScript) con la traducción española de ese párrafo.
  Requiere navegador moderno (Chrome/Edge 114+, Safari 17+, Firefox 125+).
- `hpmor-bilingue.epub` — EPUB 3 estándar: un XHTML por capítulo, limpio, y
  todas las traducciones agrupadas al final del libro (un fichero por capítulo,
  bajo la entrada «Traducciones» del índice). Los marcadores usan
  `epub:type="noteref"`, así que los lectores con soporte (Apple Books, Thorium,
  Moon+ Reader) muestran la nota como pop-up nativo; en el resto funciona como
  enlace a las notas finales con enlace de retorno.

## Uso

```sh
cargo build --release
./target/release/bilingue   # desde la raíz del repositorio
```

Genera ambos ficheros en unos segundos.

## Cómo empareja los párrafos

Por **posición**: antes de tocar nada comprueba que las secuencias de bloques
(`p`, `hr`, `div`, `blockquote`, `ol`, `ul`, `h1`–`h3`, excluidos los
`<aside>`) de ambos documentos son idénticas en número y orden. Si una futura
edición rompe esa igualdad, el programa aborta con error: nunca produce un
libro silenciosamente desalineado.

Llevan marca solo los `<p>` con texto visible cuyo párrafo español emparejado
no está vacío. Las notas contienen el HTML español verbatim, marcado con
`lang="es"`.

## Decisiones registradas (entrevista de diseño)

1. HTML primario + EPUB nativo desde el mismo binario (sin Calibre).
2. HTML: marcador `<button>` (los `<a>` no son invocadores de `popovertarget`)
   y popover anclado con CSS Anchor Positioning (`position-area: center top`):
   se abre justo encima de su párrafo, a su mismo ancho. Sin soporte de
   anclas, degrada a pop-up centrado en el viewport.
3. EPUB: corte por `<h2>` y por los `<h1>` de arco; nav a dos niveles.
4. Marca: superíndice `*` tras la puntuación, idéntica en ambos formatos.
5. Notas finales del libro (no por capítulo): un fichero `notasNNN.xhtml` por
   capítulo con notas, agrupadas bajo la entrada «Traducciones» del nav.
   Ids globales (`en-nota-N`, `es-nota-N`), enlaces entre ficheros y backlink
   obligatorio.
6. Título interno fiel al inglés, sin subtítulos añadidos.
7. Portadas del EPUB: `hpmor-bilingue-cover.jpg` para el bilingüe y
   `hpmor-portada.jpg` para la edición en español; si existen en la raíz, se
   incrustan como `cover-image` (página `cover.xhtml` primera del spine y
   entrada en el nav). Si falta alguna, esa edición se genera sin portada.
8. Salidas versionadas junto a las entradas; sin commit automático.
