---
name: traduccion-espanol-rae
description: Traduce, revisa y corrige textos en español de España conforme a la norma RAE/ASALE, preservando significado, registro, formato y terminología. Úsala para traducir, revisar traducciones o corregir ortografía y estilo («revisa este texto», «corrige la ortografía», «adapta al español»); no impone una reescritura creativa si no se solicita.
---

# Traducción al español de España conforme a la RAE

Aplica esta skill cuando el usuario pida traducir un texto al español, revisar una traducción o corregir un texto en español para ajustarlo a la norma RAE/ASALE y al uso estándar del español de España.

## Resultado esperado

Entrega una traducción fiel, natural y clara en español de España. Usa un registro estándar general, sin localismos regionales salvo que el usuario los pida. «Conforme a la RAE» significa seguir la norma académica; no conviertas automáticamente el texto en español de otro país. Si el usuario indica otra variante hispánica, respétala explícitamente.

Preserva el sentido, la intención, el grado de certeza, el tono, el registro y la estructura del original. No añadas explicaciones, matices, advertencias ni información que no esté en la fuente. No suavices ni intensifiques el contenido por iniciativa propia.

## Procedimiento

1. Identifica, cuando sea relevante, la lengua de origen, el público, la variante regional, el ámbito técnico y el registro. Si no se especifica una variante, usa español de España. Si falta un dato que cambia materialmente la traducción, formula una pregunta breve; en los demás casos, decide y continúa.
2. Traduce por sentido y contexto, no palabra por palabra. Evita calcos sintácticos y semánticos, falsos amigos y anglicismos innecesarios, pero conserva un término extranjero, una marca o una jerga cuando sean parte del significado o del registro.
3. Conserva sin alterarlos el formato, los párrafos, las listas, los títulos, las etiquetas, los enlaces, las variables, los marcadores, las unidades, las cifras, las citas y el contenido de código. Traduce esos elementos solo si el usuario lo pide y la sustitución es segura.
4. Si el usuario aporta un glosario o este existe en el proyecto, sus términos prevalecen sobre la preferencia general.
5. Haz una revisión normativa y de estilo:
   - ortografía, tildes, diéresis, grafías y separación de palabras;
   - concordancia, régimen preposicional, pronombres, tiempos verbales y construcciones sintácticas;
   - puntuación y tipografía españolas, incluidos «¿?» y «¡!» cuando correspondan;
   - mayúsculas y minúsculas: no conserves por reflejo las mayúsculas del inglés en días, meses, lenguas, gentilicios, cargos o títulos, salvo nombres propios y denominaciones oficiales;
   - léxico natural en España, evitando falsos amigos y usos propios de otras variantes cuando exista una opción estándar española adecuada;
   - consistencia de nombres, términos repetidos, números y unidades.
6. Compara la versión final con la fuente para detectar omisiones, adiciones, cambios de alcance, ambigüedades o contradicciones.

## Reglas normativas frecuentes

- Extranjerismos: escríbelos en cursiva (o entre comillas si no hay formato) con su grafía original: *software*, *marketing*. No los adaptes a la grafía española.
- Comillas: prefiere las latinas «»; anida, por este orden, “” y luego ‘’.
- Números: escribe con letras los que caben en una o dos palabras (*veintinueve*, *trescientos*); usa cifras en datos técnicos, cifras de más de dos palabras y series.
- Fechas y horas: «12 de agosto de 2026», con el mes en minúscula y sin ordinal; hora con dos puntos, «17:30».
- Porcentajes, monedas y unidades: deja espacio entre cifra y símbolo: «20 %», «50 km», «30 €».

## Ejemplos

| Original | Corrección |
| --- | --- |
| The meeting is on August 12 at 5:30 PM. | La reunión es el 12 de agosto a las 17:30. |
| Guarda el file en tu Desktop folder. | Guarda el archivo en tu carpeta *Desktop*. |
| El Software incluye 30 days of Free Trial. | El *software* incluye 30 días de prueba gratuita. |

## Fuentes y decisiones dudosas

Cuando una elección normativa o léxica no sea segura, consulta primero las fuentes oficiales de RAE/ASALE, especialmente el [Diccionario panhispánico de dudas](https://www.rae.es/dpd/), el [Diccionario de la lengua española](https://dle.rae.es/) y la [*Ortografía de la lengua española*](https://www.rae.es/ortografia/). El DPD es la referencia prioritaria para dudas de uso, sintaxis, puntuación, extranjerismos, calcos y variantes; la *Ortografía*, para tildes, números y signos. Para elegir entre variantes admitidas, prioriza la solución propia del español estándar de España solicitada por esta skill. No presentes una preferencia estilística como una prohibición académica.

Si existen varias soluciones correctas, elige la más natural para un lector de España. Si la variante regional modifica una palabra o construcción importante, ofrece la alternativa solo cuando el usuario la pida o cuando aporte valor, y señala brevemente la diferencia.

## Forma de respuesta

Por defecto, devuelve únicamente la traducción, sin prefacios ni comentarios. Conserva el formato solicitado por el usuario. Añade notas breves solo si las pide o si una ambigüedad del original exige dejar constancia de la decisión; en ese caso, separa claramente la traducción de las notas.

No afirmes que el resultado está «certificado por la RAE». Puedes indicar que se ha revisado conforme a criterios y fuentes de RAE/ASALE cuando sea pertinente.
