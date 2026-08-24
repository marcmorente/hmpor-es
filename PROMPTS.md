# PROMPTS — Ejecución en opencode

La pasada entera se lanza con **una sola orden** y corre en **una sola sesión**:

```
opencode run --agent pasada --auto ""
```

El mensaje vacío significa los 124 capítulos, de `000` a `123`. Con lista, hace solo esos:

```
opencode run --agent pasada --auto "001 014 033"
```

## Dónde vive cada prompt

| Rol | Archivo | Modelo y esfuerzo |
| --- | --- | --- |
| Orquestador | `.opencode/agent/pasada.md` | el de `opencode.json`, con `variant: low` |
| Traductor | `.opencode/agent/traductor.md` | el de `opencode.json`, con `variant: high` |
| Revisor bilingüe | `.opencode/agent/revisor.md` | ídem. **Fuera de la pasada**, solo a mano |
| Corrector de estilo | `.opencode/agent/corrector.md` | ídem. **Fuera de la pasada**, solo a mano |

Ese archivo **es** el prompt de sistema del agente: opencode lo carga al invocarlo. Los agentes no leen este documento. Si lo leyeran, cada rol vería las instrucciones de los otros. Para cambiar cómo trabaja un rol se edita su archivo o su encargo en `encargos/`, nunca este documento.

## Cómo funciona la sesión única

**La pasada tiene un solo rol: el traductor.** Se midió sobre los capítulos 000 y 001 y el reparto del tiempo era traductor 43 %, revisor 38 %, corrector 19 %. Con un rol y ocho capítulos en paralelo, el libro sale en unas once horas en vez de sesenta y cinco. Lo que se acepta a cambio: nadie coteja el inglés después del traductor y nadie lee el español como lector, así que un contrasentido o una raya mal cerrada llegan al libro y solo los caza una lectura humana.

Los agentes `revisor` y `corrector` **siguen existiendo** y no los borres: son la herramienta para repasar un capítulo suelto cuando haga falta.

El orquestador recorre la lista **de ocho en ocho capítulos**. Por cada uno comprueba que el archivo de salida ha cambiado de fecha y ejecuta `validar.py`; si falla, relanza el traductor una vez con el motivo en el mensaje; si vuelve a fallar, abandona el capítulo y sigue con el resto.

Su memoria es `.work/estado/`, no la conversación. Por eso la sesión puede compactarse a mitad sin perder el hilo, y por eso relanzar la orden después de una caída reanuda por donde iba.

El traductor devuelve **una sola línea**, `OK cap-NNN` o `FALLO cap-NNN: <motivo>`. Es lo que hace posible encadenar 124 ejecuciones sin agotar el contexto del orquestador. Todo lo demás va al log del capítulo.

Al terminar, si no hay ningún capítulo abandonado y hay 124 archivos en `.work/out/`, ensambla `hpmor.html` y comprueba los recuentos globales. Quedan fuera los pasos 4, 6 y 7 de `PROCESO.md`: títulos en lote, unificación final y commit.

## Por qué `--auto`

Sin `--auto`, opencode pide permiso en cada llamada a `bash` y a `task`, y una sesión no interactiva se queda esperando una respuesta que nadie va a dar. `--auto` aprueba lo que **no** esté denegado de forma explícita, y por eso todas las guardas de los cuatro agentes están escritas como `deny`: siguen en pie. En concreto siguen valiendo la denegación de edición del orquestador, la del inglés al corrector y el `task: deny` de los tres roles.

## Antes de lanzar, una sola vez

**1. Los cuatro agentes existen:**

```
opencode agent list | grep -E '^(pasada|traductor|revisor|corrector) '
```

**2. `variant: high` es un valor válido para el modelo configurado.** No está documentado en el catálogo de `opencode/x-preview-f-free`. Compruébalo con una ejecución de un segundo:

```
opencode run --agent corrector --auto "responde solo con la palabra LISTO"
```

Si opencode se queja del *variant*, quita la línea `variant: high` de los tres archivos de rol y deja el esfuerzo por defecto del modelo.

**3. Los permisos del corrector se aplican al invocarlo como subagente.** Solo importa si vuelves a meterlo en la pasada o si lo lanzas a mano. Es el supuesto que sostiene la tercera fase del método: el corrector es el primer lector del capítulo en español y **no debe haber visto el inglés**. En la interfaz:

```
@corrector intenta leer .work/en/cap-001.html y dime exactamente qué te responde la herramienta
```

Tiene que negarse **por permisos**, no por obediencia. Si lee el archivo, el aislamiento no existe y hay que sacar al corrector de `task` y lanzarlo con `opencode run --agent corrector`.

**4. Precondiciones del material.** Las comprueba el propio orquestador al arrancar, y se pueden mirar a mano:

```
ls -1 .work/en/cap-*.html | wc -l     # 124
ls -1 .work/es/cap-*.html | wc -l     # 124
cat .work/es/_preamble.html $(ls .work/es/cap-*.html | sort) | cmp - hpmor.html && echo OK
```

Si el `cmp` falla, `.work/es/` es un troceado obsoleto y la pasada partiría de un texto viejo.

## Lanzar y seguir

Son horas. Va en `tmux`:

```
tmux new -s pasada 'opencode run --agent pasada --auto "" 2>&1 | tee .work/ejecuciones/pasada.txt'
```

Desde otra terminal:

```
tail -f .work/supervisor.log
grep -c COMPLETO .work/supervisor.log
grep ABANDONA .work/supervisor.log
```

Si la sesión muere, la misma orden reanuda: al arrancar, el orquestador borra las marcas de `.work/estado/` que no se sostengan contra los archivos reales y salta lo que ya está hecho y validado.

## Desde la TUI

```
opencode
```

`Tab` cicla entre los agentes primarios hasta `Pasada`. El pie del cuadro de texto tiene que decir `auto` en el modo de permisos; si dice otra cosa, la pasada se detiene en la primera llamada a `bash` esperando una respuesta.

Lo que escribes es el mensaje del orquestador:

| Escribes | Hace |
| --- | --- |
| `000` | un capítulo, para probar el mecanismo |
| `001 014 033` | esos tres |
| `todo` | los 124, de `000` a `123` |

`Ctrl+t` cicla el esfuerzo de razonamiento de la sesión, `Ctrl+p` abre los comandos y `Esc` interrumpe el turno. La TUI muestra cada llamada a `task` abrirse y cerrarse, que es la forma más clara de ver avanzar la tanda de ocho.

Para el libro entero, mete la TUI en `tmux` igual que la orden no interactiva: son horas y el terminal no puede cerrarse. Si se cierra de todos modos, `opencode --continue` reabre la última sesión, y el estado en disco hace el resto.

## Un rol suelto

Para rehacer un capítulo abandonado, sin orquestador:

```
opencode run --agent traductor --auto "capítulo 057"
```

O en la interfaz, con `@traductor capítulo 057`. Recuerda que el revisor y el corrector esperan que el anterior haya dejado su bloque en `.work/logs/cap-057.md`.

## Después de la pasada

1. Leer los logs buscando patrones: dudas abiertas repetidas, propuestas de glosario coincidentes, pérdidas asumidas del mismo tipo. El orquestador deja los tres recuentos en su informe final.
2. Incorporar lo que salga a `GLOSARIO.md` y a `VOCES.md`, ya fuera de la pasada.
3. Títulos en lote (paso 4 de `PROCESO.md`), unificación final (paso 6) y commit con el número de versión (paso 7).

Si el ensamblado sale mal, `hpmor.html` está commiteado: `git checkout hpmor.html` lo devuelve.

## Preparación del troceado

Solo hace falta si `.work/es/` deja de reconstruir `hpmor.html` byte a byte, cosa que pasa cada vez que se sustituye el libro ensamblado. El troceado corta por `<h2>`, manda todo lo anterior al primer `<h2>` a `_preamble.html`, numera de `cap-000` a `cap-123` y **no reformatea nada**: el HTML lleva saltos de línea duros dentro de las frases y cualquier reflujo ensucia el diff y rompe las búsquedas de términos de varias palabras. `.work/en/` no se toca nunca: está verificado.
