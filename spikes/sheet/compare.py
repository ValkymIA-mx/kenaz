#!/usr/bin/env python3
"""Compare stable content and page geometry; not a visual or QR scan test."""
from pathlib import Path
import re
import subprocess

root = Path(__file__).resolve().parents[2]
out = root / 'spikes/sheet/target/sheet-sample'
markers = {
    'en': ['Recovery sheet', 'Write by hand', 'Certificates (PIV)', 'GitHub', 'ana@example.com', 'Passkeys on this key', 'If something goes wrong'],
    'es': ['Hoja de recuperación', 'Escribe a mano', 'Certificados (PIV)', 'GitHub', 'ana@example.com', 'Si algo sale mal'],
}

def run(*args):
    return subprocess.check_output(args, text=True)

def info(path):
    text = run('pdfinfo', str(path))
    pages = re.search(r'^Pages:\s*(\d+)', text, re.M)
    size = re.search(r'^Page size:\s*([\d.]+) x ([\d.]+) pts', text, re.M)
    if not pages or not size:
        raise ValueError(f'cannot read page geometry: {path}')
    return int(pages[1]), float(size[1]), float(size[2])

for lang in ('es', 'en'):
    reference = root / f'templates/recovery-sheet/example-{lang}.pdf'
    reference_info = info(reference)
    for rendered in (out / f'example-{lang}.pdf', out / f'embedded-{lang}.pdf'):
        rendered_info = info(rendered)
        if reference_info != rendered_info:
            raise ValueError(f'{rendered}: page geometry differs: {reference_info} vs {rendered_info}')
        for pdf in (reference, rendered):
            text = ' '.join(run('pdftotext', str(pdf), '-').split())
            for marker in markers[lang]:
                if marker not in text:
                    raise ValueError(f'{pdf}: missing {marker!r}')
        print(f'{rendered}: reference page geometry and selected text agree: {rendered_info}')
