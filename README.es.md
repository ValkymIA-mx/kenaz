# Kenaz ᚲ

**ValkymIA Kenaz — tu llavero de seguridad.**

[English](README.md)

Kenaz es una app de escritorio libre y de código abierto que guía a cualquier persona para proteger sus cuentas con una llave de seguridad física (YubiKey): llaves de acceso (passkeys), códigos de 6 dígitos y una llave de respaldo para que perder una no sea un desastre. Explica, en lenguaje llano, qué hacer con la llave, en qué orden y cómo no quedarse fuera.

> **Estado: experimentos de M0 en curso.** Todavía no hay app para instalar. Hay experimentos independientes, prototipos en papel en ambos idiomas y borradores de contenido, además del diseño y la plantilla de recuperación. Consulta el [informe de M0](docs/reports/2026-11-09-m0-progress.md). Kenaz es una herramienta comunitaria con soporte best-effort (sin garantías).

## En qué te ayuda

### Primero: proteger tus cuentas

- **Entra con tu llave.** Agrega la llave a tus cuentas importantes como llave de acceso (sin contraseña) o como segundo paso después de tu contraseña. Primero el correo, luego el banco, luego las redes sociales.
- **Códigos de 6 dígitos.** Escanea el código QR que te da el sitio y obtén tus códigos desde la llave; se copian con un clic.
- **Una llave de respaldo desde el primer día.** Todo se guarda en dos llaves, así que perder una no te deja fuera. Kenaz lleva una lista de los sitios a los que aún les falta la llave de respaldo.
- **«¿Estoy protegido?»** Una sola pantalla muestra qué sitios tienen las dos llaves, qué falta y cuántos intentos de PIN quedan.
- **Cuando algo sale mal.** Pasos guiados para reemplazar una llave perdida, desbloquear un PIN o restablecer una llave que vas a regalar, con avisos claros antes de borrar nada.

### Después, con tus cuentas ya protegidas

- Abrir un gestor de contraseñas con la llave (primero KeePassXC).
- Iniciar sesión en tu computadora, usar SSH y firmar PDF.
- Cifrar y firmar correo (primero Thunderbird).
- Evitar que la llave escriba letras al azar cuando la tocas.
- Firmar PDF con tu e.firma del SAT guardada en la llave.

## Cómo evita que te quedes fuera

- **Dos llaves, configuradas juntas.** Los secretos no se pueden copiar de una YubiKey después, así que Kenaz escribe cada uno en las dos llaves a la vez.
- **Un archivo de secretos cifrado.** Los secretos de configuración se guardan en un solo archivo [age](https://age-encryption.org/v1) que solo tus llaves pueden abrir, para poder preparar una llave de reemplazo a partir de él. También se puede abrir fuera de Kenaz con las herramientas estándar de age.
- **Una hoja de recuperación para imprimir**, una por llave, por si se pierden las llaves y el archivo. Recupera tus códigos de 6 dígitos y el desbloqueo del gestor de contraseñas, pero no tus llaves de cifrado de correo ni de certificados: esas no se imprimen, así que se pierden junto con las llaves y el archivo (la e.firma se puede volver a importar desde los archivos originales del SAT). Los PIN y los códigos de administración nunca se imprimen: los escribes a mano.
- **Un solo PIN que recordar.** Los códigos de administración son aleatorios, se guardan en el archivo de secretos y se muestran una sola vez para la hoja de recuperación.

Las llaves de acceso no se pueden respaldar así: cada sitio web registra cada llave por separado. Kenaz te guía para agregar la llave de respaldo en cada sitio y te avisa en cuáles falta.

## Para quién es

- Personas que usan la misma contraseña en todas partes y nunca han activado la verificación en dos pasos.
- Personas a las que les dieron una YubiKey y no saben qué hacer con ella.
- Personas que ya protegen sus cuentas y quieren dar el siguiente paso.

No es para especialistas en seguridad, organizaciones que reparten llaves a su personal ni configuraciones de OpenPGP a la medida: para eso ya existen [Yubico Authenticator](https://www.yubico.com/products/yubico-authenticator/), Kleopatra y `ykman`. Kenaz es un acompañante guiado, no su reemplazo; las mismas llaves siguen funcionando con las herramientas oficiales.

## Plataformas y llaves

| | 1.0 | Más adelante |
|---|---|---|
| Linux (Wayland), x86-64 y ARM64 | ✓ | |
| Windows, x86-64 y ARM64 | ✓ | |
| macOS | | Necesita una persona externa que colabore |
| Android, iOS | | En el plan |

- **Llaves probadas:** YubiKey 5 y Security Key.
- **Sin probar:** YubiKey 4 y YubiKey 5 con firmware anterior a 5.7. Kenaz lee lo que admite cada llave y solo ofrece eso, pero estos casos no se prueban con hardware.
- **Idiomas:** español e inglés.

## El nombre

**Kenaz (ᚲ)** es la runa de la antorcha: conocimiento e iluminación. Kenaz ilumina, en lenguaje llano, un tema opaco para quien no es técnico: qué hacer con la llave, en qué orden y cómo no quedarse fuera.

**Pronunciación:** KE-nas (es) · KEN-az (en).

## Documentos del proyecto

Los documentos de diseño están en inglés.

- [`IDEA.md`](IDEA.md): diseño del producto: alcance, principios, flujos, reglas de respaldo, tecnología.
- [`docs/roadmap/overview.md`](docs/roadmap/overview.md): hitos, líneas de trabajo, decisiones pendientes, plan de pruebas.
- [`docs/decisions/`](docs/decisions/): decisiones registradas.
- [`docs/runbooks/dev-environment.md`](docs/runbooks/dev-environment.md): herramientas de compilación y laboratorio de máquinas virtuales.
- [`templates/recovery-sheet/`](templates/recovery-sheet/): la hoja de recuperación para imprimir (Typst).

Hecho con Rust, [Slint](https://slint.dev) y el crate [`yubikit`](https://docs.rs/yubikit) de Yubico.

## Soporte

Kenaz es un proyecto comunitario. La ayuda es best-effort: sin tiempo de respuesta garantizado y sin garantía. Si te quedaste fuera de una cuenta, la recuperación de cuenta del propio sitio es el camino más rápido.

## Licencia

Código abierto bajo [GPL-3.0-only](LICENSE), compatible con los términos con los que el proyecto usa Slint. Las fuentes incluidas conservan sus propias licencias en `spikes/sheet/fonts/`.

Kenaz no está afiliado ni respaldado por Yubico. YubiKey y Yubico son marcas registradas de Yubico AB.
