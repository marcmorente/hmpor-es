#!/usr/bin/env python3
"""Validación estructural de un capítulo traducido.

Uso: validar.py NNN
Salida: 0 si el capítulo pasa; 1 con el motivo en stderr si falla.
Los avisos justificables van a stdout y no hacen fallar la validación.
"""
import re, sys, pathlib

ESTRUCTURA = ['<p', '<hr', '<div', '<blockquote', '<ol', '<ul', 'lettrine', 'smallcaps']

# Formas que el glosario prohibe tras las decisiones cerradas.
PROHIBIDAS = ['Mortífago', 'Giratiempos', 'legilimancia', 'Legilimancia', 'maestro de Pociones',
              'Maestro de Pociones', 'Legión del Caos', 'Comed-Tea', 'Britania', 'Transfiguración',
              'Señor Oscuro', 'Niño-Que-Vivió', 'Niño que Vivió', 'Regimiento Soleado',
              'Departamento de Aplicación de la Ley']

def sin_notas(t):
    """El bloque <aside> de notas no cuenta para los recuentos estructurales."""
    return re.sub(r'<aside\b.*?</aside>', '', t, flags=re.S)

def main():
    if len(sys.argv) != 2:
        print('uso: validar.py NNN', file=sys.stderr); return 1
    c = sys.argv[1]
    en_p = pathlib.Path(f'.work/en/cap-{c}.html')
    out_p = pathlib.Path(f'.work/out/cap-{c}.html')
    for p in (en_p, out_p):
        if not p.is_file() or p.stat().st_size == 0:
            print(f'falta o está vacío {p}', file=sys.stderr); return 1

    en = en_p.read_text(encoding='utf-8')
    out = out_p.read_text(encoding='utf-8')
    out_sin = sin_notas(out)
    plano = re.sub(r'\s+', ' ', re.sub(r'<[^>]*>', ' ', out))
    fallos, avisos = [], []

    # 1. recuentos estructurales
    for t in ESTRUCTURA:
        a, b = en.count(t), out_sin.count(t)
        if a != b:
            fallos.append(f'{t}: inglés {a}, salida {b}')

    # 2. terminos prohibidos por el glosario
    for t in PROHIBIDAS:
        n = len(re.findall(re.escape(t), plano))
        if n:
            fallos.append(f'término prohibido «{t}» ×{n}')

    # 3. notas: cada llamada con su nota y al reves
    refs = set(re.findall(r'href="#(n\d{3}-\d+)"', out))
    notas = set(re.findall(r'<aside[^>]*id="(n\d{3}-\d+)"', out))
    if refs - notas:
        fallos.append(f'llamadas sin nota: {sorted(refs - notas)}')
    if notas - refs:
        fallos.append(f'notas sin llamada: {sorted(notas - refs)}')

    # 4. log del capitulo
    log = pathlib.Path(f'.work/logs/cap-{c}.md')
    if not log.is_file():
        fallos.append('falta el log del capítulo')

    # --- avisos, no bloquean ---
    a, b = en.count('<em'), out_sin.count('<em')
    if a != b:
        avisos.append(f'<em>: inglés {a}, salida {b} (desvío justificable, debe estar explicado en el log)')
    if len(notas) > 2:
        avisos.append(f'{len(notas)} notas del traductor, por encima del orientativo 0-2')

    for a_ in avisos:
        print(f'AVISO cap-{c}: {a_}')
    if fallos:
        for f in fallos:
            print(f'FALLO cap-{c}: {f}', file=sys.stderr)
        return 1
    print(f'VALIDO cap-{c}')
    return 0

if __name__ == '__main__':
    sys.exit(main())
