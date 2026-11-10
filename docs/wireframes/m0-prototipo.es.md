# M0 — Prototipo en papel (español)

**Borrador para U0; textos y navegación sin validar.** No representa una aplicación funcional ni decisiones de M1. Imprime o copia cada sección H1–H10 como una tarjeta. La persona facilitadora cambia de tarjeta según la ruta indicada; los nombres de acciones son texto impreso, no botones interactivos. Usa fichas de papel para las llaves y tarjetas aparte para el navegador y la impresora. Los números entre corchetes son referencias para facilitar, no texto de producto. «Principal» y «repuesto» son nombres provisionales, no una decisión sobre cómo distinguirlas (D17).

## Mapa de pantallas

- Inicio H1 → primera configuración H2–H4 (W1); elegir cuenta H5 → H6 → H7 o H8 → H9–H10 (W2). De H1 también se llega a códigos, estado, lista, reemplazo y detalles de H11. Volver a Inicio está disponible en cada sección.
- H11 → lista de pendientes → terminar repuesto: cuentas de acceso vuelven a H6–H7; secretos guardados se completan desde el archivo. H11 → reemplazar llave → recorrido W6 de abajo. La lista muestra lo detectable y permite agregar manualmente lo que no se detecta.
- Después de proteger la primera cuenta, Inicio ofrece las tareas posteriores W8–W13; antes no aparecen. W6 y W7 están disponibles como recuperación, no como tareas posteriores. Este mapa no fija disposición, estilo ni ilustraciones (D13–D16 pendientes).

## H1 — Inicio

**¿Qué quieres proteger?**

Empieza por tu correo: ayuda a recuperar tus otras cuentas. Tú eliges qué cuenta proteger.

- Preparar mis llaves → H2 (si aún no están preparadas)
- Proteger una cuenta → H5
- Obtener un código de 6 dígitos → H11, Códigos
- ¿Estoy protegido? → H11, Estado
- Ver mi lista y terminar el repuesto → H11, Lista
- Perdí o se dañó una llave → H11, Reemplazo
- Olvidé mi PIN o está bloqueado → H11, PIN
- Detalles avanzados → H11, Detalles

Si no hay repuesto: **Todavía no tienes una llave de repuesto. Si pierdes esta llave, podrías perder el acceso.** → Preparar repuesto / Continuar sin repuesto.

Solo después de proteger la primera cuenta: «Mi llave escribe letras», «Abrir mi gestor de contraseñas», «Entrar a mi computadora», «Cifrar y firmar correo», «Regalar o borrar una llave» y «Firmar PDF con mi e.firma» → tareas W8–W13; no se desarrollan en este prototipo.

## H2 — Tu primera llave (W1)

Conecta tu YubiKey principal. La comprobaremos antes de continuar.

Esperando una llave… → si se reconoce, H3. Si no es una YubiKey: «Esta no es una YubiKey compatible. Conecta una YubiKey.» Si otro programa la usa: «Otro programa está usando tu llave. Ciérralo y vuelve a intentar.» Si falta acceso o el servicio: «No podemos acceder a la llave. Revisa la conexión y los permisos o el servicio de llaves.» → Reintentar / Volver a Inicio.

## H3 — Elige tu PIN

Elige un PIN de 6 a 8 caracteres: letras sin acentos, números o símbolos ASCII. No uses ñ ni letras acentuadas. Escríbelo de nuevo para confirmarlo. No lo compartas.

Si no cumple la regla o no coincide: «Revisa el PIN y vuelve a escribirlo.» Si la llave tiene reglas adicionales, se explican antes de guardar. Si quedan menos intentos del máximo: «Quedan [n] intentos; si se agotan, puedes bloquear la llave.» → Continuar / Volver.

**Para facilitar:** D8 sigue pendiente: esta tarjeta no promete cuándo se cambian los PIN de fábrica de PIV/OpenPGP ni sus códigos administrativos. No crear archivo de secretos aquí.

## H4 — Prepara el repuesto

Conecta tu segunda YubiKey para usar el mismo PIN. Si solo tienes un puerto, desconecta la principal y conecta el repuesto. Si se pide, toca la llave indicada. Espera a que termine; no la desconectes mientras se guarda.

- Segunda llave lista → H5.
- No tengo repuesto → «Puedes continuar, pero si pierdes tu llave podrías perder el acceso. Algunas cosas no se pueden copiar después.» → Continuar sin repuesto → H5 / Conectar repuesto.
- Llave incorrecta, ocupada o desconectada durante la operación → «No terminamos de preparar esta llave. Conecta la llave indicada y vuelve a intentar.» No afirmar que quedó lista → Reintentar / Volver a Inicio.

## H5 — Elige una cuenta (W2)

¿Qué cuenta quieres proteger? Sugerimos correo primero, luego banca y después redes sociales. Puedes elegir otra o escribir su nombre.

Correo / Banca / Redes sociales / Escribir sitio → Continuar. Si el sitio no está en la lista: «Te mostraremos pasos generales; el sitio puede llamarlos de otra forma.» → H6.

## H6 — En el sitio

Abre la configuración de seguridad de [sitio] en tu navegador. Sigue los pasos de ese sitio; Kenaz no registra la llave por ti.

1. Inicia sesión en [sitio].
2. Abre Seguridad y busca formas de iniciar sesión.
3. Sigue la opción indicada abajo. Ponle el nombre «YubiKey principal».
4. Si el navegador lo pide, conecta y toca la llave principal. Vuelve aquí cuando el sitio confirme que la agregó.

La ruta es fija según el sitio: primero acceso sin contraseña con tu llave; si no lo admite, llave después de tu contraseña; si tampoco lo admite, códigos de 6 dígitos → H8. Acceso con llave → H7. No fingir confirmación del sitio.

**Espacio:** al añadir la primera llave de acceso, «Tus llaves pueden guardar hasta [menor capacidad] accesos así. Muchos sitios no usan este espacio.» Si hay dos llaves y una anterior a 5.7 ya guarda 20 accesos, antes de cada sitio: «Quedan [menor espacio libre] espacios; los usan [sitios].» Muestra que el dato de una llave ausente es de [fecha]. La persona puede usar un espacio o elegir llave después de contraseña si el sitio lo ofrece; no se borra nada automáticamente. Si el navegador indica que la llave está llena: «No hay espacio. Revisa tus accesos; antes de quitar uno, asegúrate de tener otra forma de entrar.» → Lista / Volver.

## H7 — Ahora agrega el repuesto

En el mismo sitio, agrega otra llave y llámala «YubiKey repuesto». Con un solo puerto, desconecta la principal y conecta el repuesto; si se solicita, toca el repuesto. Vuelve cuando el sitio confirme que lo agregó. Cada llave se registra por separado.

- El sitio confirmó ambas → H9.
- No ahora / no tengo repuesto → «Esta cuenta todavía no tiene repuesto. Si pierdes la llave principal, podrías perder el acceso.» → H9 con «falta repuesto» en la lista.
- El sitio falla o la llave se desconecta → «No se confirmó el repuesto.» → Reintentar / Dejar pendiente → H9 con «falta repuesto».

## H8 — Códigos de 6 dígitos

En el sitio, abre la configuración de códigos. Captura el QR de la pantalla o pega el enlace de configuración. La primera vez, elige una o más formas disponibles de abrir tu archivo cifrado de secretos; cualquiera de las elegidas lo abre. Si ninguna funciona con tus llaves: «No podemos guardar un archivo que se abra con tus llaves. Para usar un repuesto, debes completar ambas llaves antes de cerrar la aplicación.» → Continuar en memoria / Cancelar.

Conecta la principal; si se pide, tócala. «Guardando en la llave principal… No la desconectes.» Después de escribir en ella: «Guardando en el archivo cifrado…» Solo cuando se confirma el guardado, conecta el repuesto (desconecta la principal primero si hay un puerto), tócalo si se pide y espera a que termine. Si no hay repuesto o falla después de guardar: «El repuesto está pendiente; podrás terminarlo con el archivo cifrado.» → H9 pendiente. Si se cierra antes de confirmar el archivo: «No podrás terminar el repuesto desde el archivo; tendrás que configurar de nuevo.» Nunca se guarda el secreto sin cifrar.

Sin archivo, si se usa repuesto ambas llaves necesitan espacio y el repuesto debe escribirse antes de cerrar; si se interrumpe, no prometer recuperación desde archivo. Se puede continuar solo con la principal, con la advertencia de que no habrá copia para completar el repuesto después. Si la principal está llena: «No hay espacio; no se escribió ni guardó nada.» Si el repuesto está lleno, avisar antes de escribir cuando se conozca: «Puedes hacer espacio o continuar con el repuesto pendiente.» Con archivo, se escribe la principal y se guarda el archivo; la lista dice «falta repuesto: sin espacio». Sin archivo y con repuesto, no se escribe nada hasta que ambas tengan espacio. Si el repuesto no admite códigos: «Esta llave no admite códigos; el repuesto quedará pendiente.» Sin archivo, no prometer que se pueda terminar después. → H9 pendiente / Cancelar. Tras la primera cuenta de códigos, ofrecer hoja de recuperación → H10.

## H9 — Códigos de respaldo del sitio

¿El sitio te dio códigos de respaldo? Imprímelos desde el sitio o cópialos a mano y guárdalos con tu hoja de recuperación. Kenaz no los guarda ni los muestra. Si imprimes, usa el navegador y recoge las hojas de la impresora.

Ya terminé / No me dio códigos → H10. Si solo hay una llave: «Falta el repuesto; perder la llave puede dejarte sin acceso.»

## H10 — Tu lista

[Cuenta]: ambas llaves / falta repuesto / falta en esta llave, según lo confirmado. «Una cuenta de acceso con llave necesita que la agregues por separado en cada llave.» Las cuentas que no se pueden detectar se agregan y marcan a mano.

Ofrecer hoja de recuperación si corresponde: «Imprime una hoja por llave y guárdala en un lugar seguro. Escribe a mano los códigos privados que se muestren; nunca se imprimen el PIN ni los códigos que permiten cambiarlo.» → Imprimir: salir a la impresora, recoger las hojas / Ahora no. → Inicio H1 / Proteger otra cuenta H5 / Lista H11.

## H11 — Otros destinos del mapa

- **Códigos (W3):** «Conecta una llave para ver tus códigos.» Lista de códigos de la llave conectada → Copiar; si lo pide, «Toca tu llave». Vacío: «No hay cuentas de códigos en esta llave.»
- **Estado (W5):** número de sitios con ambas llaves y pendientes, espacios usados por cada llave, antigüedad de la última lectura si está ausente, copia externa del archivo cifrado y los intentos de PIN restantes por llave. Si no se sabe, mostrar «desconocido», no inventar un número.
- **Lista (W4):** sitios «ambas llaves», «falta repuesto» o «falta en esta llave»; agregar y marcar manualmente los sitios que no se detectan. Un secreto guardado se completa desde el archivo; un sitio con llave requiere volver al navegador y registrar el repuesto por separado. → H6 o H8 según corresponda.
- **Reemplazo (W6):** seguir el recorrido de abajo. **PIN (W7):** advertir intentos y consecuencias antes de cualquier acción; un reinicio de acceso invalida todos los accesos de esa llave. Comprobar abriendo el archivo con un método que sobreviva antes de cualquier reinicio; si ninguno lo abre, detenerse.
- **Detalles avanzados:** términos reales: llave de acceso = passkey, llave después de contraseña = security key, códigos = OATH TOTP, archivo cifrado = age; opciones de apertura OpenPGP, PIV o FIDO2. No supone una recomendación D2.

## Recorrido y traspasos — W1, W2 y W6

| Momento | Persona / sistema | Salida o estado |
|---|---|---|
| W1 | Conecta principal → verifica → elige PIN → conecta repuesto o desconecta principal y cambia → toca si se solicita; o salta repuesto | H2–H4; advertencia persistente si falta |
| W2 acceso | Elige sitio → sale al navegador → conecta/toca principal si se solicita → vuelve → desconecta/cambia o conecta repuesto → vuelve al navegador y toca si se solicita → vuelve | H5–H7; confirmación por llave, o pendiente |
| W2 códigos | Sale al navegador para QR/enlace → vuelve, elige apertura de archivo si es la primera vez → conecta/toca principal → escribe principal → confirma guardado cifrado → cambia/conecta y toca repuesto si se solicita | H8; excepción sin archivo y errores descritos allí |
| W2 final | Sale al navegador para imprimir o copiar a mano códigos del sitio → recoge impresos; si ofrece hoja, sale a impresora y recoge una por llave → vuelve | H9–H10; lista según confirmaciones |
| W6 1 | **Primero:** abre correo en navegador y quita la llave perdida; después la quita de los demás sitios de la lista. Usa la llave sobreviviente o el método de recuperación del sitio para entrar; vuelve a la app | Si no puede entrar, usar recuperación del sitio; no afirmar que se quitó |
| W6 2 | Conecta sobreviviente y nueva; con un puerto, desconecta y cambia cuando se indique. Establece el mismo PIN en la nueva; toca cuando se solicite | Si falta la sobreviviente o falla el PIN, detener y explicar; no afirmar restauración |
| W6 3 | Abre el archivo con la sobreviviente; toca si se solicita. Cambia/conecta la nueva y escribe códigos, identidades OpenPGP/PIV y secretos de botón disponibles | Si no se abre el archivo, detener restauración y explicar que no puede copiar esos secretos desde la llave; usar recuperación disponible |
| W6 4 | Los accesos con llave **no** se restauran del archivo. Cada sitio queda «falta en la llave nueva». Abre cada sitio en navegador, entra con sobreviviente u otro método, cambia a nueva y tócala si se pide para registrarla; vuelve y marca solo tras confirmación | Algunos sitios retrasan el uso: Google puede tardar hasta 7 días. Si falla, queda pendiente |
| W6 5 | Si el sitio ofrece códigos nuevos, imprimir o copiar a mano; recoger impresos. Cuando corresponda, imprimir y recoger hoja de la llave nueva | Lista y hoja reflejan solo lo completado |
