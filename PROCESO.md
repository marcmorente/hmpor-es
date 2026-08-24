# PROCESO — Flujo de la pasada

Este documento describe el flujo. No lo implementa: los scripts se escriben después del piloto, cuando el flujo ya esté probado.

## Materiales

| Archivo | Qué es |
| --- | --- |
| `hpmor-en.html` | Original inglés. Fuente de verdad del contenido. |
| `hpmor.html` | Traducción actual (v0.02). Línea base y referencia. |
| `ESTILO.md` | Norma común de los tres roles. |
| `encargos/01-traductor.md`, `02-revisor.md`, `03-corrector.md` | Encargo de cada rol. |
| `GLOSARIO.md` | Términos vinculantes. Solo lectura durante la pasada. |
| `VOCES.md` | Voz de cada personaje. Solo lectura durante la pasada. |
| `.work/en/`, `.work/es/`, `.work/out/`, `.work/logs/` | Trabajo por capítulos. |

Estructura del libro: 7 `<h1>` (portada y seis arcos) y 125 `<h2>` (Prefacio, Introducción del autor y 123 capítulos).

## Paso 0 — Glosario y voces

Antes de la pasada:

1. Se extraen candidatos de tres fuentes: términos y nombres del inglés por frecuencia, sus soluciones actuales en el español con el recuento de variantes, y las dudas registradas en los 123 logs de la pasada anterior.
2. Se verifica el canon de Salamandra para lo que existe en Rowling.
3. Se validan a mano las decisiones abiertas, incluida la de «mortífago» frente a «Mortífago», que la pasada anterior fijó en mayúscula en 183 sitios.
4. Se escribe `VOCES.md` con los personajes de más diálogo.

Estos dos archivos quedan **congelados** al arrancar la pasada. Ningún agente los edita.

## Paso 1 — Troceado

`hpmor.html` se vuelve a partir en `.work/es/cap-NNN.html`, uno por `<h2>`, más `_preamble.html`. `.work/en/` se regenera igual desde `hpmor-en.html`. `.work/out/` se vacía. `.work/logs/` **se conserva**: los bloques nuevos se añaden a los existentes.

El corte se hace por el encabezado, sin reformatear el resto del archivo. El HTML lleva saltos de línea duros dentro de las frases: no se normalizan, porque cualquier reflujo ensucia el diff de git.

## Paso 2 — Los tres roles, en cadena

Por cada capítulo, tres ejecuciones en este orden, cada una con su encargo:

1. **Traductor**: `.work/en/cap-NNN.html` + `.work/es/cap-NNN.html` → `.work/out/cap-NNN.html`.
2. **Revisor bilingüe**: `.work/en/cap-NNN.html` + `.work/out/cap-NNN.html` → `.work/out/cap-NNN.html`.
3. **Corrector de estilo**: `.work/out/cap-NNN.html` → `.work/out/cap-NNN.html`.

Cada rol añade su bloque al log del capítulo. Ninguno borra lo anterior.

Los capítulos pueden ir en paralelo entre sí. Los tres roles de un mismo capítulo, no: van en cadena.

El revisor no lee el log del traductor antes de cotejar. El corrector no abre el inglés. Son condiciones del método, no recomendaciones.

## Paso 3 — Validación por capítulo

Después de cada rol:

**Fallo duro** si no se cumple algo de esto:

- Recuentos iguales al capítulo inglés, excluido el bloque `<aside>` de notas, para: `<p>`, `<hr>`, `<div>`, `<blockquote>`, `<ol>`, `<ul>`, `<span class="lettrine">`, `<span class="smallcaps">`.
- Sin texto inglés sin traducir.
- Términos del glosario en su forma única.
- Llamadas y notas emparejadas, con identificadores `nNNN-N` correctos.

**Aviso justificable**, que no bloquea pero exige explicación en el log:

- Desvío en el número de `<em>` frente al inglés.
- Más de dos notas del traductor.
- Cambio de tratamiento dentro de una escena.

Un capítulo con fallo duro vuelve al rol que lo produjo, con el motivo. No pasa al rol siguiente.

## Paso 4 — Títulos, en lote aparte

Los 6 títulos de arco y los 123 de capítulo se revisan en **una sola ejecución**, con la lista completa delante, para que sean coherentes entre sí y con el glosario. Ningún agente de capítulo toca su propio título.

El paratexto (Prefacio e Introducción del autor, 627 palabras) pasa por los tres roles como si fuera un capítulo. El aviso legal y de licencia se traduce con literalidad, sin criterio de estilo.

## Paso 5 — Ensamblado

`_preamble.html` y los 124 fragmentos de `.work/out/` se concatenan en orden y sustituyen a `hpmor.html`.

Comprobación global antes de dar por buena la fusión: recuento de etiquetas del libro ensamblado frente al inglés, y recuento de `<h1>` y `<h2>` frente a los valores conocidos (7 y 125).

## Paso 6 — Unificación final

Con el libro ensamblado se recuentan las variantes de cada término del glosario y de cada nombre inventado, y se corrigen las divergencias hacia la forma fijada.

Este paso es el que caza lo que los agentes en paralelo no pueden ver. En el texto actual, por ejemplo, convivían «Regimiento del Sol» y «Regimiento Sol», y «Legión del Caos» aparecía 36 veces frente a 51 apariciones del término inglés.

**Cuidado con los recuentos.** El HTML lleva saltos de línea dentro de las frases, así que cualquier búsqueda de más de una palabra da resultados bajos si no se normalizan los espacios antes. Toda comprobación de términos de varias palabras se hace sobre el texto con los saltos convertidos en espacios.

## Paso 7 — Cierre

1. Los logs se revisan buscando patrones: dudas abiertas repetidas, propuestas de glosario coincidentes, pérdidas asumidas en el mismo tipo de pasaje.
2. Lo que salga de ahí se incorpora a `GLOSARIO.md` y a `VOCES.md`, ya fuera de la pasada.

## Piloto

Antes de la pasada completa se prueban **tres capítulos** de dificultad distinta:

- uno inicial y sencillo, para calibrar la voz base y la ortotipografía;
- uno con diálogo de varios personajes a la vez, para poner a prueba la matriz de tratamiento y las voces;
- uno con juegos de palabras y razonamiento científico, para poner a prueba el glosario técnico, la jerarquía de compensación y el límite de notas.

Se leen los resultados, se ajustan los documentos y solo entonces se lanza el libro. El piloto cuesta 9 ejecuciones; la pasada completa, 372.
