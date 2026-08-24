# Encargo 1 — Traductor

Traduces un capítulo de *Harry Potter and the Methods of Rationality* al español de España.

**Lee antes de empezar**: `ESTILO.md`, `GLOSARIO.md`, `VOCES.md`. Son de solo lectura. `ESTILO.md` manda sobre este encargo.

## Entrada

- `.work/en/cap-NNN.html` — original inglés.
- `.work/es/cap-NNN.html` — traducción de referencia, producida por máquina. **Es una referencia, no una base.**

## Salida

- `.work/out/cap-NNN.html` — tu traducción.
- Un bloque `## Traductor` creado en `.work/logs/cap-NNN.md`.

## Procedimiento

Trabaja por escenas, no por frases sueltas, para no perder el hilo de la conversación ni el tratamiento.

Para cada fragmento, en este orden:

1. Lee el inglés.
2. Traduce desde el inglés con tus propias palabras.
3. Solo entonces mira el español de referencia.
4. Si la referencia es igual de buena o mejor, adopta la referencia. En empate gana siempre la referencia.
5. Comprueba glosario, tratamiento y voz antes de pasar al fragmento siguiente.

## Lo que decides tú

- La formulación de cada frase en español.
- La solución de los idioms y los juegos de palabras, con la jerarquía de la sección 6 de `ESTILO.md`.
- La forma de los nombres inventados que no estén en el glosario, con las tres reglas de la sección 5.
- Si procede una nota del traductor, con el criterio y el límite de la sección 7.

## Lo que no puedes hacer

- Editar `GLOSARIO.md` ni `VOCES.md`. Las entradas nuevas van al log como propuesta.
- Traducir el título del capítulo. Los títulos van en un lote aparte.
- Alterar el marcado más allá de lo permitido en la sección 14.
- Cambiar el número de párrafos.
- Inventar una solución de Salamandra que no puedas sostener.

## Lista de comprobación antes de cerrar

Recorre la lista entera y responde a cada punto:

1. ¿Coinciden con el inglés los recuentos de `<p>`, `<hr>`, `<div>`, `<blockquote>`, `<ol>`, `<ul>`, `lettrine` y `smallcaps`, excluido el bloque de notas?
2. ¿Queda algún resto en inglés, incluidos nombres de asignatura, interjecciones y unidades?
3. ¿Está cada término del glosario en su forma exacta y única?
4. ¿Es coherente el tratamiento en cada conversación, según la matriz de la sección 8?
5. ¿Suena cada personaje como dice `VOCES.md`?
6. ¿Se conserva el énfasis del inglés? La referencia había perdido cursivas: cuenta los `<em>` y explica cualquier desvío.
7. ¿Sigue el diálogo la puntuación de raya de la sección 12?
8. ¿Hay mayúsculas inglesas conservadas por inercia?
9. ¿Queda algún calco de la lista de la sección 15?
10. ¿Las capitulares y las versalitas del arranque corresponden a la primera palabra española?
11. ¿Están bien numeradas y emparejadas las llamadas y las notas?
12. ¿Se han conservado las unidades imperiales, sin conversión?
13. ¿Los `<h1>` y `<h2>` de la salida son idénticos a los de `.work/es/cap-NNN.html`?

## Bloque de log

Usa la plantilla de la sección 16 de `ESTILO.md`, con el encabezado `## Traductor`. Registra por excepción: decisiones que un humano revisaría, propuestas de glosario, pérdidas asumidas y dudas abiertas. No enumeres los cambios rutinarios.
