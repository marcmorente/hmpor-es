# ESTILO — Norma común de traducción

Este documento obliga a los tres roles del proyecto: **traductor**, **revisor bilingüe** y **corrector de estilo**. Cada rol tiene además su propio encargo en `encargos/`, que fija sus límites y su lista de comprobación. Cuando el encargo y este documento parezcan chocar, manda este documento.

Datos del proyecto: `GLOSARIO.md` (términos vinculantes) y `VOCES.md` (voz de cada personaje). El proceso está descrito en `PROCESO.md`.

---

## 1. Qué se traduce y con qué criterio general

*Harry Potter and the Methods of Rationality* es un fanfic de Eliezer Yudkowsky ambientado en el mundo de J. K. Rowling. El lector de destino es un lector español que ha leído la saga en la edición de Salamandra. Por eso el texto en español debe cumplir dos condiciones a la vez:

1. **Sonar a Hogwarts en español**, es decir, al mundo tal como lo nombró Salamandra.
2. **Sonar a novela escrita en español**, no a traducción.

El original mezcla dos registros que hay que mantener separados: la narración de novela juvenil británica y el discurso técnico de un niño que razona en voz alta sobre ciencia. Ninguno de los dos se contagia al otro.

## 2. La tarea: retraducción referenciada

Existe una traducción previa al español, producida por máquina. **No es la base del trabajo: es una referencia.**

Procedimiento obligatorio, en este orden:

1. Lee el fragmento en inglés.
2. **Traduce desde el inglés** con tus propias palabras.
3. **Solo entonces** compara con el español de referencia.
4. Si la solución de la referencia es igual de buena o mejor que la tuya, **adopta la de la referencia**.
5. Si la tuya es mejor, usa la tuya.

El orden importa. Leer primero el español ancla la sintaxis inglesa que arrastra la referencia, que es justo el defecto que hay que eliminar.

**Regla de empate**: en igualdad de calidad gana siempre lo que ya existe. Esto protege la coherencia con los capítulos que otros agentes traducen en paralelo.

**La regla de empate obliga solo al traductor**, y el motivo no es jerárquico: es que solo él tiene delante el inglés y el español de referencia a la vez. Sin los dos no se puede saber si hay empate.

- El **revisor bilingüe** puede invocarla, porque también tiene el original.
- El **corrector de estilo no la aplica nunca.** No ve el inglés, así que una solución de la referencia que le parezca equivalente puede estar destruyendo un matiz que solo se ve en el original. Si una frase del traductor le parece peor que la de la referencia, **no la sustituye**: la anota en «Discrepancias con otro rol» y decide el revisor bilingüe, que es quien puede comparar.

**Herencia obligatoria**: los nombres, términos y juegos de palabras ya resueltos se heredan. La fuente de esas decisiones es `GLOSARIO.md`. Si un término aparece en la referencia pero no en el glosario, se conserva la forma de la referencia y se anota en el log.

## 3. Invariantes

Estos límites no se negocian nunca, con ningún argumento de estilo:

- **No se añade contenido.** Ni aclaraciones, ni matices, ni conectores que expliquen lo que el original deja implícito.
- **No se suprime contenido.** Ninguna frase, imagen ni chiste se cae por ser difícil.
- **No se reordena la información** entre párrafos. Dentro de la frase, el orden se adapta al español; entre frases y párrafos, no.
- **No se cambia el número de párrafos.** Un `<p>` del inglés es un `<p>` del español.
- **No se toca el marcado** salvo lo permitido en la sección 14.
- **No se cambia el grado de certeza ni la intensidad.** Si el personaje duda, duda igual; si insulta, insulta igual.
- **No se suaviza.** El original tiene crueldad, muerte y humor negro. Se traducen.

## 4. Canon: jerarquía de fuentes

Orden de prelación cuando dos criterios chocan:

1. **`GLOSARIO.md`** del proyecto.
2. **Canon de Salamandra** para lo que existe en Rowling y no está en el glosario.
3. **Norma RAE/ASALE**: *Diccionario de la lengua española*, *Diccionario panhispánico de dudas* y *Ortografía de la lengua española*.
4. **Naturalidad en español de España**.
5. **Literalidad del inglés**.

Reglas de uso:

- Durante la pasada, **`GLOSARIO.md` es de solo lectura**. No lo edites por ningún motivo. Todos los agentes trabajan a la vez y tienen que ver lo mismo.
- **Prohibido inventar canon.** No atribuyas a Salamandra una solución que no puedas sostener. Si no sabes cómo tradujo Salamandra un término, no lo adivines con aire de autoridad: aplica la regla 4 o 5 y anótalo en el log como duda abierta.
- Si crees que el glosario contiene un error, **cúmplelo igualmente** y anótalo en el log. La corrección es una decisión editorial que se toma fuera de la pasada.
- Español de **España**, sin localismos. Voseo y léxico americano quedan excluidos.

## 5. Nombres inventados de HPMOR

El fanfic crea nombres que no existen en Rowling. Se resuelven con tres reglas, por este orden:

1. **Antropónimo o topónimo → se mantiene en inglés.** Los nombres de persona y de lugar no se traducen, igual que Salamandra mantuvo *Hogwarts*, *Hogsmeade* o *Dumbledore*.
2. **Nombre descriptivo o institucional → se traduce.** Si el nombre describe lo que la cosa es (un regimiento, una legión, una sociedad, un objeto con función), pasa al español con la misma soltura con la que Salamandra resolvió *Room of Requirement* como «Sala de los Menesteres» o *Pensieve* como «pensadero».
3. **Nombre con chiste, sigla o anagrama → se recrea.** Se conserva la función, no la letra. El precedente de la casa es doble: *S.P.E.W.* se convirtió en **P.E.D.D.O.** conservando la sigla ridícula, y el anagrama de *Tom Marvolo Riddle* obligó a alterar el nombre propio hasta que el anagrama funcionase en español. Se admite el mismo grado de libertad.

Procedimiento para cada nombre nuevo:

1. Determina qué hace el nombre: nombrar, describir o hacer gracia.
2. Aplica la regla que corresponda.
3. Comprueba que la forma elegida **se pueda declinar y pluralizar** con naturalidad en español y que no choque con otra ya usada.
4. Anota la decisión en el log como propuesta de entrada de glosario, con la forma exacta y su plural.

Las mayúsculas siguen la norma española, no la inglesa: en español los nombres comunes que forman parte de una denominación no se escriben con mayúscula por imitación del inglés.

## 6. Idioms, chistes y juegos de palabras

Nunca se traduce un idiom palabra por palabra. La jerarquía de técnicas es cerrada y se aplica en este orden:

1. **Equivalente español** que cumpla la misma función en el mismo sitio.
2. **Recreación con otro recurso.** Un juego fónico puede volverse léxico; una rima puede volverse aliteración. Lo que se conserva es el efecto sobre el lector, no el mecanismo.
3. **Desplazamiento.** Si el juego no cabe donde está, se compensa unas líneas más allá, dentro de la misma escena.
4. **Literal con pérdida asumida**, solo cuando fallan las tres anteriores. Se anota la pérdida en el log.

Prohibiciones:

- **No expliques el chiste dentro del texto.** Nada de glosas ni aposiciones aclaratorias.
- **No conviertas un juego en información.** Si el original bromea, el español bromea o calla; no informa.
- Si el juego sostiene la trama y ninguna técnica funciona, procede la nota del traductor de la sección 7.

Casos frecuentes en este libro que exigen decisión consciente:

- Chistes que dependen de la etimología de un nombre propio.
- Chistes que dependen de la ortografía o la fonética inglesas.
- Chistes que dependen de un concepto científico. Aquí manda el concepto: si hay que elegir, se salva la exactitud del razonamiento y se compensa el humor en la frase siguiente.
- Verso, canciones y fórmulas rimadas: se traduce con rima o ritmo equivalentes; si es imposible, se busca ritmo sin rima antes que prosa plana.

## 7. Notas del traductor

Están permitidas, con criterio cerrado y límite duro.

**Procede la nota solo si se cumplen las dos condiciones**:

1. La pérdida impide entender la trama o el chiste central de la escena.
2. Ninguna técnica de la sección 6 la ha podido evitar.

**No procede la nota**:

- para explicar cultura británica que el lector deduce del contexto;
- para explicar un concepto científico o matemático;
- para justificar la traducción elegida;
- para dar la conversión de una unidad imperial;
- para señalar un guiño a la saga de Rowling.

**Límite**: 0-2 notas por capítulo. Pasar de dos exige justificarlo en el log, entrada por entrada.

**Redacción**: una o dos frases, en español neutro y sin humor propio. Empieza por `<em>N. del T.</em>: `. La nota informa y desaparece; no comenta ni opina.

**Marcado exacto**. La llamada, en el sitio del texto donde estaba el juego:

```html
<a id="n042-1-ref" href="#n042-1" epub:type="noteref" role="doc-noteref"><sup>1</sup></a>
```

El bloque de notas, **al final del capítulo**, después del último párrafo:

```html
<aside id="n042-1" epub:type="footnote" role="doc-footnote">
<p><a href="#n042-1-ref">1.</a> <em>N. del T.</em>: texto de la nota.</p>
</aside>
```

Reglas del identificador: `n` + número de capítulo con tres cifras + guion + número de nota dentro del capítulo. La numeración de notas empieza en 1 en cada capítulo.

Este marcado se muestra como nota emergente en Apple Books y Kindle, y cae con elegancia a nota al final del capítulo en los lectores que no lo admiten. El bloque `<aside>` es la **única** excepción a la prohibición de añadir marcado.

## 8. Tratamiento: matriz de tú y usted

El tratamiento es la primera cosa que se desmorona cuando muchos agentes trabajan en paralelo. Esta matriz no admite interpretación.

| Quién habla | A quién | Tratamiento |
| --- | --- | --- |
| Alumno | Profesor, adulto con cargo | usted |
| Profesor | Alumno | tú, con «señor Potter» / «señorita Granger» donde el inglés usa *Mr.* / *Miss* |
| Alumno | Alumno, incluidos rivales | tú |
| Harry | Dumbledore | usted |
| Dumbledore | Harry y alumnos | tú |
| Snape | Alumnos | tú (el desprecio va en el léxico, no en el tratamiento) |
| McGonagall | Alumnos | tú + «señor» / «señorita» |
| Hagrid | Alumnos | tú |
| Quirrell | Harry | **usted**, como marca deliberada de trato entre iguales |
| Harry | Quirrell | usted |
| Lucius Malfoy | Harry | usted (distancia amenazante) |
| Draco | Adultos ajenos a la familia | usted |
| Padres de Harry | Harry, y Harry a ellos | tú |
| Elfos domésticos | Magos | usted |
| Duendes de Gringotts | Clientes | usted |
| Voldemort | Cualquiera | tú (desprecio) |
| Retratos y fantasmas | Alumnos | tú |
| Funcionario o auror en acto oficial | Cualquiera | usted |

**Regla que resuelve los casos no tabulados**: con Harry, el tratamiento de un adulto codifica **cómo lo considera**. El adulto que lo trata como niño usa tú (McGonagall, Hagrid, Pomfrey). El adulto que lo trata como interlocutor con poder usa usted (Quirrell, Lucius, cargos del Ministerio). Elige según lo que el personaje esté haciendo en esa escena, y mantén la elección.

**Estabilidad**: dentro de una misma conversación no se cambia de tratamiento. Un cambio solo se admite si el inglés lo marca y significa algo (un personaje que pasa a la amenaza, a la intimidad o al desprecio). Todo cambio de tratamiento dentro de una escena se anota en el log.

**Plural**: vosotros y las formas en `-áis` / `-éis` para el plural informal. `Ustedes` solo en trato formal a un grupo: discurso oficial, tribunal, adulto que marca distancia con desconocidos.

## 9. Voces de los personajes

`VOCES.md` fija, para los personajes con más diálogo, el nivel de léxico, la longitud de frase típica, las muletillas y las prohibiciones. Es de lectura obligatoria y de solo lectura.

Tres reglas generales:

- **Harry habla como un niño de once años con vocabulario de adulto lector**, no como un adulto. Cuando usa un término técnico, lo usa bien; cuando se enfada, se enfada como un niño.
- **El habla marcada se marca con sintaxis y léxico, no con ortografía fonética.** Nada de apóstrofos ni de vocales comidas para imitar acento popular.
- **Los verbos de habla no se adornan.** Si el inglés repite *said*, el español repite «dijo». No se sustituye por «exclamó», «profirió» ni «espetó» para variar.

## 10. Terminología científica y racionalista

La mitad del libro es razonamiento. La terminología se traduce con la forma **consolidada en español en su campo**, no con la más parecida al inglés. Estas formas son vinculantes:

| Inglés | Español |
| --- | --- |
| prior, prior probability | probabilidad previa |
| posterior probability | probabilidad posterior |
| to update (beliefs) | actualizar (creencias) |
| likelihood | verosimilitud |
| Bayes' theorem | teorema de Bayes |
| Bayesian | bayesiano |
| expected utility | utilidad esperada |
| utility function | función de utilidad |
| expected value | valor esperado |
| decision theory | teoría de la decisión |
| game theory | teoría de juegos |
| prisoner's dilemma | dilema del prisionero |
| Occam's razor | navaja de Occam |
| hypothesis | hipótesis |
| falsifiable | falsable |
| bias | sesgo |
| heuristic | heurística |
| control group | grupo de control |
| confounder | variable de confusión |
| efficient market hypothesis | hipótesis de los mercados eficientes |
| many-worlds interpretation | interpretación de los muchos mundos |
| conservation of energy | conservación de la energía |
| self-fulfilling prophecy | profecía autocumplida |

**Desdoble por registro**. Algunos términos tienen dos soluciones legítimas según el contexto, y elegir mal delata la traducción automática:

- *evidence* → **«evidencia»** dentro de un razonamiento probabilístico explícito; **«pruebas»** o **«indicios»** en diálogo corriente y en contexto judicial.
- *argument* → **«argumento»** cuando es un razonamiento; **«discusión»** cuando es una pelea.
- *assume* → **«suponer»** o **«dar por supuesto»**; nunca «asumir» con el sentido inglés.
- *realize* → **«darse cuenta»** o **«comprender»**; «realizar» solo significa hacer.
- *eventually* → **«al final»**, **«con el tiempo»**; nunca «eventualmente».
- *actually* → **«en realidad»**; nunca «actualmente».
- *sensible* → **«sensato»**; *sensitive* → «sensible».
- *predict* → **«predecir»**; *prediction* → «predicción».

Los símbolos, fórmulas y cifras no se tocan.

## 11. Realia: unidades, monedas, fechas

- **Sistema imperial siempre**: millas, yardas, pies, pulgadas, libras, onzas, galones. **No se convierten** a métrico, ni en el texto ni en nota. Son color local británico, y es el criterio de Salamandra. Si la referencia trae kilómetros, kilos o centímetros, se deshace la conversión.
- **Temperatura**: se conserva la escala del original.
- **Monedas**: galeón, sickle, knut, con sus plurales españoles («galeones», «sickles», «knuts»); libras esterlinas para el dinero muggle.
- **Fechas**: «12 de agosto de 1991», mes en minúscula, sin ordinales.
- **Horas**: cifras en narración técnica («las 17:30»), letras en diálogo corriente («las cinco y media»).
- **Instituciones, festividades y realia británica** conservan su nombre: no se sustituye una referencia británica por una española equivalente.
- **Porcentajes y unidades**: espacio entre cifra y símbolo, «20 %», «50 km».
- **Números**: con letras los que caben en una o dos palabras; con cifras los datos técnicos y las series.

## 12. Ortotipografía española

### Diálogo

Se usa **raya** (—, U+2014), no comillas y no guion corto.

- La raya de apertura va pegada a la primera palabra: `—No lo sé.`
- El comentario del narrador con verbo de habla se introduce con raya pegada y va en minúscula: `—No lo sé —dijo Harry.`
- Si el diálogo continúa después del comentario, se cierra la raya y el punto va **después**: `—No lo sé —dijo Harry—. Lo preguntaré mañana.`
- Si el comentario no lleva verbo de habla, el diálogo se cierra con punto **antes** de la raya: `—No lo sé. —Se encogió de hombros.`
- Si el diálogo termina ahí, no se cierra la raya.
- Cada intervención empieza en párrafo propio, y ese párrafo es el `<p>` que ya existe. No se abre uno nuevo.

**Narración y diálogo en el mismo párrafo.** El inglés los junta a menudo en un solo párrafo: «The Professor rolled his eyes. “Dear, I understand that you’re not familiar…”». El español los pondría en dos líneas, pero el invariante de la sección 3 lo prohíbe: un `<p>` del inglés es un `<p>` del español. **Se resuelve dentro del párrafo, sin partirlo.**

- Narración primero: se cierra con punto y la intervención empieza con raya pegada a la primera palabra, después de un espacio. `El profesor puso los ojos en blanco. —Cariño, entiendo que no estás familiarizada…`
- Intervención primero y luego comentario sin verbo de habla: el diálogo se cierra con punto **antes** de la raya, como en la regla general. `—No lo sé. —Se encogió de hombros.`
- **No se disimula convirtiendo la narración en verbo de habla.** `—Cariño, entiendo… —dijo el profesor poniendo los ojos en blanco` reordena información entre oraciones y lo prohíbe la sección 3.

Es el caso más frecuente del libro y **no es un defecto de la traducción**: es la consecuencia de conservar la estructura de párrafos del original. No se anota en el log ni se marca como duda.

### Comillas y cursiva

- Comillas **latinas** «» en primer nivel; “” en segundo; ‘’ en tercero.
- **Cursiva** (`<em>`) para: énfasis, extranjerismos, hechizos, títulos de obra y pensamiento directo.
- **Los hechizos van en cursiva** y conservan su forma latina original.
- No se usan comillas para el pensamiento (véase la sección 13).

### Mayúsculas

El inglés escribe con mayúscula mucho más que el español. Se corrige por norma española, no por imitación:

- **minúscula**: días, meses, estaciones, gentilicios, idiomas, cargos y tratamientos («señor Potter», «profesora McGonagall», «el ministro», «el director»).
- **mayúscula**: nombres propios, nombres de las casas, denominaciones oficiales de instituciones y asignaturas del colegio.
- Los epítetos y apodos siguen la forma que fije `GLOSARIO.md`, en una sola variante para todo el libro.

### Puntuación

- Apertura obligatoria de interrogación y exclamación, también en frases mixtas.
- Puntos suspensivos: tres, sin espacio antes.
- Incisos: rayas o paréntesis; nunca guion corto ni semiguion.
- Coma antes de vocativo y detrás de él.
- Sin coma entre sujeto y verbo.
- Sin coma antes de «y» en enumeración simple (nada de coma serial inglesa).

## 13. Pensamiento y voces internas

- **Pensamiento directo**: cursiva, sin comillas y sin verbo introductorio si el original no lo lleva.
- **Voces internas** (Ravenclaw, Slytherin, Hufflepuff, Gryffindor y las demás que discuten dentro de la cabeza de Harry): se tratan **como interlocutores reales**, con raya de diálogo y verbo de habla, y el nombre de la casa funciona como nombre propio de personaje.
- Las cursivas del inglés **se conservan**. Pueden cambiar de sitio dentro del párrafo si el español coloca el énfasis en otra palabra, pero no se eliminan. La traducción de referencia perdió énfasis por el camino: recupéralo.

## 14. Marcado HTML

El archivo es XHTML generado con pandoc. Cada capítulo llega como fragmento.

**Se conserva exactamente**, en número y en orden: `<h1>`, `<h2>`, `<p>`, `<hr>`, `<br>`, `<div>`, `<blockquote>`, `<ol>`, `<ul>`, `<a>`, `<strong>`, `<span class="lettrine">`, `<span class="smallcaps">`.

**Se permite**:

- Mover un `<em>` dentro de su párrafo.
- Recortar de nuevo `lettrine` y `smallcaps` al inicio del capítulo. La capitular contiene la **primera letra de la primera palabra española** y las versalitas contienen **el resto de esa palabra o las primeras palabras**, según lo que haya en el inglés. Nunca dejes esos `<span>` vacíos, con una letra que no corresponda o con texto en inglés.
- Añadir el bloque `<aside>` de notas de la sección 7.

**Está prohibido**: añadir clases, atributos, estilos, identificadores ajenos a las notas, comentarios HTML, saltos de línea decorativos y espacios de no separación.

Los encabezados `<h1>` y `<h2>` del capítulo se copian **tal cual** del español de referencia, letra por letra. Ya están traducidos y su revisión va en un lote aparte. Traducirlos del inglés es un error: la salida debe tener encabezados idénticos a los de `.work/es/cap-NNN.html`.

## 15. Calcos que hay que eliminar

La referencia viene de traducción automática y arrastra estos defectos. Búscalos activamente:

1. **Posesivo redundante**: «levantó su mano» → «levantó la mano».
2. **Sujeto pronominal innecesario**: «Él sabía que…» → «Sabía que…».
3. **Gerundio calcado** del *-ing* inglés, y gerundio de posterioridad: se resuelve con subordinada o con dos oraciones.
4. **Pasiva perifrástica** donde el español usa activa o impersonal: «fue visto por Harry» → «Harry lo vio».
5. **Orden de palabras inglés**: adverbio al principio, complemento antes del verbo, adjetivo antepuesto sin motivo.
6. **Perífrasis progresiva** («estaba siendo», «está siendo») donde el español usa un tiempo simple.
7. **Falsos amigos** de la sección 10.
8. **Muletillas traducidas una a una**: *Oh*, *Well*, *You know*, *I mean*. En español suelen desaparecer o reducirse a «Bueno», «Ya», «O sea». No se calcan todas.
9. **Coletillas interrogativas** (*isn't it?*, *right?*) → «¿no?», «¿verdad?».
10. **Mayúsculas inglesas** conservadas por inercia.
11. **Puntuación inglesa**: coma serial, comillas rectas, guion corto por raya.
12. **Preposición calcada**: régimen preposicional inglés sobre verbo español.

## 16. Log del capítulo

El log de cada capítulo empieza vacío: los de la pasada anterior se han retirado a propósito, para que sus decisiones no sesguen esta.

El **traductor crea** `.work/logs/cap-NNN.md`. El revisor y el corrector **añaden** su bloque al final. Ningún rol toca ni reescribe el bloque de otro.

Plantilla del bloque:

```markdown
## <rol>

- Cambios por categoría: sentido N | canon N | ortotipografía N | voz N | juego de palabras N
- Notas añadidas: N
- Dudas abiertas: N
- Validación: <correcta | fallida, con el motivo>

### Decisiones
- **<término o fragmento>**: qué se ha decidido y por qué, con la norma o la entrada de glosario que lo respalda.

### Propuestas de glosario
- **<inglés>** → **<español>** (plural: <forma>). Motivo.

### Pérdidas asumidas
- **<fragmento>**: qué se ha perdido y por qué no había alternativa.

### Dudas abiertas
- **<término o pasaje>**: qué falta para decidir.

### Discrepancias con otro rol
- **<fragmento>**: qué cambió el rol anterior, qué se ha hecho y con qué fundamento.
```

Se registra **por excepción**: solo lo que un humano necesitaría revisar. No enumeres los cambios rutinarios de sintaxis ni de puntuación: el diff completo lo guarda git. Si una sección no tiene contenido, escribe `- (ninguna)`.

## 17. Cierre del capítulo

Un capítulo no está terminado si falla cualquiera de estas comprobaciones.

**Fallo duro**:

- El número de `<p>`, `<hr>`, `<div>`, `<blockquote>`, `<ol>`, `<ul>`, `<span class="lettrine">` y `<span class="smallcaps">` no coincide con el del capítulo inglés, excluido el bloque `<aside>` de notas.
- Queda texto en inglés sin traducir.
- Hay un término del glosario en una forma distinta de la fijada.
- El bloque `<aside>` tiene llamadas sin nota o notas sin llamada.

**Aviso justificable** (no bloquea, pero se explica en el log):

- El número de `<em>` se desvía del inglés.
- Hay más de dos notas del traductor.
- Hay un cambio de tratamiento dentro de una escena.

## 18. Prohibiciones absolutas

1. No editar `GLOSARIO.md` ni `VOCES.md`.
2. No inventar canon de Salamandra.
3. No añadir ni quitar contenido.
4. No cambiar el número de párrafos.
5. No añadir marcado fuera del bloque de notas.
6. No explicar chistes dentro del texto.
7. No convertir unidades imperiales.
8. No usar léxico ni gramática de variedades americanas.
9. No suavizar violencia, crueldad ni humor negro.
10. No borrar ni reescribir lo que otro rol dejó en el log.
