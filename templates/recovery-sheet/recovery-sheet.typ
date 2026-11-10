// Recovery sheet for one YubiKey.
//
// Compile:
//   typst compile recovery-sheet.typ --input data=example.json out.pdf
//
// The PIN and PUK are never part of the data: the sheet prints empty boxes
// and the user writes them by hand.

#let data = json(sys.inputs.at("data", default: "example.json"))
#let lang = sys.inputs.at("lang", default: data.at("lang", default: "es"))

#let strings = (
  es: (
    title: "Hoja de recuperación",
    subtitle: "Todo lo necesario para recuperar o reemplazar esta llave",
    warning-title: "Guárdala como tu pasaporte",
    warning: "Con esta hoja y la llave, cualquier persona puede entrar a tus cuentas y firmar documentos a tu nombre. No le tomes foto, no la escanees y no la guardes en la computadora o en la nube. Cuando imprimas una hoja nueva para esta llave, destruye esta.",
    generated: "Impresa el",
    sheet-id: "Hoja",
    page: "Página",
    of: "de",
    by-hand: "Escribe a mano",
    by-hand-note: "Estos datos nunca se imprimen. Escríbelos con pluma, una letra por casilla.",
    pin: "PIN",
    pin-note: "El mismo para llaves de acceso, firma y cifrado (OpenPGP) y certificados (PIV). 6 a 8 caracteres.",
    puk: "PUK",
    puk-note: "Desbloquea el PIN de certificados (PIV). Con él se puede poner un PIN nuevo: protégelo igual que el PIN.",
    copy-location: "Copia del archivo de secretos guardada en",
    device: "La llave",
    label: "Nombre",
    role: "Papel",
    role-main: "Llave principal",
    role-spare: "Llave de repuesto",
    model: "Modelo",
    serial: "Número de serie",
    firmware: "Firmware",
    form-factor: "Formato",
    nfc: "NFC",
    yes: "Sí",
    no: "No",
    piv: "Certificados (PIV)",
    management-key: "Clave de administración",
    algorithm: "Algoritmo",
    management-key-note: "También está guardada en la llave, protegida por el PIN. Otras herramientas (ykman, Yubico Authenticator) pueden pedirla para cambiar certificados.",
    slot: "Ranura",
    purpose: "Uso",
    key-type: "Tipo de llave",
    origin: "Origen",
    origin-generated: "Creada por la app",
    origin-imported: "Importada",
    origin-efirma: "e.firma del SAT (importada)",
    subject: "Titular",
    issuer: "Emisor",
    cert-serial: "Número de certificado",
    fingerprint: "Huella SHA-256",
    valid: "Vigencia",
    to: "a",
    openpgp: "Firma y cifrado de correo (OpenPGP)",
    admin-pin: "PIN de administrador",
    admin-pin-note: "De OpenPGP. Lo creó la app; cópialo de la pantalla. Con él se puede poner un PIN nuevo: protégelo igual que el PIN.",
    reset-code: "Código de restablecimiento",
    reset-code-note: "De OpenPGP. Lo creó la app; cópialo de la pantalla. Con él se puede poner un PIN nuevo: protégelo igual que el PIN.",
    cardholder: "Titular",
    email: "Correo",
    key-sig: "Firma",
    key-enc: "Cifrado",
    key-aut: "Autenticación",
    touch: "Tocar para usar",
    codes: "Códigos de 6 dígitos",
    codes-note: "Con el secreto se pueden generar los códigos de esa cuenta en otra llave o app.",
    account: "Cuenta",
    secret: "Secreto",
    settings: "Ajustes",
    otp: "Botón de la llave (OTP)",
    slot-short: "Toque corto",
    slot-long: "Toque largo",
    otp-empty: "Vacío",
    otp-yubico: "Contraseña de un solo uso de Yubico (de fábrica)",
    otp-chalresp: "Desafío-respuesta",
    used-for: "Se usa en",
    passkeys: "Llaves de acceso en esta llave",
    passkeys-note: "No se pueden copiar ni restaurar. Si pierdes la llave, quítala de estos sitios y registra la de reemplazo.",
    site: "Sitio",
    user: "Usuario",
    secrets-file: "Archivo de secretos",
    secrets-file-note: "Guarda las claves de certificados, de OpenPGP, los secretos de los códigos y del botón. Se abre con cualquiera de estas llaves y su PIN.",
    location: "Ubicación",
    opens-with: "Se abre con",
    recipient: "Destinatario age",
    if-trouble: "Si algo sale mal",
    trouble-app: "Primero abre Kenaz y elige «Olvidé mi PIN» o «Reemplazar una llave perdida». Si la app no está disponible:",
    trouble-piv-pin: "PIN de certificados bloqueado (usa el PUK):",
    trouble-pgp-pin: "PIN de OpenPGP bloqueado (usa el código de restablecimiento):",
    trouble-mgmt: "Cambiar certificados con otra herramienta: ykman pide la clave de administración cuando hace falta. Escríbela cuando te la pida, nunca en el comando. Por ejemplo:",
    trouble-fido: "PIN de llaves de acceso bloqueado: no hay forma de desbloquearlo. Hay que restablecer esa parte de la llave, lo que borra todas sus llaves de acceso.",
  ),
  en: (
    title: "Recovery sheet",
    subtitle: "Everything needed to recover or replace this key",
    warning-title: "Keep it like a passport",
    warning: "With this sheet and the key, anyone can sign in to your accounts and sign documents in your name. Don't photograph or scan it, and don't keep it on a computer or in the cloud. When you print a new sheet for this key, destroy this one.",
    generated: "Printed on",
    sheet-id: "Sheet",
    page: "Page",
    of: "of",
    by-hand: "Write by hand",
    by-hand-note: "These are never printed. Write them in pen, one character per box.",
    pin: "PIN",
    pin-note: "The same for passkeys, email signing and encryption (OpenPGP) and certificates (PIV). 6 to 8 characters.",
    puk: "PUK",
    puk-note: "Unblocks the certificates (PIV) PIN. It can set a new PIN: protect it like the PIN.",
    copy-location: "Copy of the secrets file kept at",
    device: "The key",
    label: "Name",
    role: "Role",
    role-main: "Main key",
    role-spare: "Spare key",
    model: "Model",
    serial: "Serial number",
    firmware: "Firmware",
    form-factor: "Form factor",
    nfc: "NFC",
    yes: "Yes",
    no: "No",
    piv: "Certificates (PIV)",
    management-key: "Management key",
    algorithm: "Algorithm",
    management-key-note: "Also stored on the key, protected by the PIN. Other tools (ykman, Yubico Authenticator) may ask for it to change certificates.",
    slot: "Slot",
    purpose: "Use",
    key-type: "Key type",
    origin: "Origin",
    origin-generated: "Created by the app",
    origin-imported: "Imported",
    origin-efirma: "SAT e.firma (imported)",
    subject: "Holder",
    issuer: "Issuer",
    cert-serial: "Certificate number",
    fingerprint: "SHA-256 fingerprint",
    valid: "Valid",
    to: "to",
    openpgp: "Email signing and encryption (OpenPGP)",
    admin-pin: "Admin PIN",
    admin-pin-note: "For OpenPGP. Made by the app; copy it from the screen. It can set a new PIN: protect it like the PIN.",
    reset-code: "Reset code",
    reset-code-note: "For OpenPGP. Made by the app; copy it from the screen. It can set a new PIN: protect it like the PIN.",
    cardholder: "Holder",
    email: "Email",
    key-sig: "Signature",
    key-enc: "Encryption",
    key-aut: "Authentication",
    touch: "Touch to use",
    codes: "6-digit codes",
    codes-note: "The secret can make that account's codes on another key or app.",
    account: "Account",
    secret: "Secret",
    settings: "Settings",
    otp: "Key button (OTP)",
    slot-short: "Short press",
    slot-long: "Long press",
    otp-empty: "Empty",
    otp-yubico: "Yubico one-time password (factory)",
    otp-chalresp: "Challenge-response",
    used-for: "Used for",
    passkeys: "Passkeys on this key",
    passkeys-note: "They can't be copied or restored. If you lose the key, remove it from these sites and register the replacement.",
    site: "Site",
    user: "User",
    secrets-file: "Secrets file",
    secrets-file-note: "Holds the certificate and OpenPGP keys and the code and button secrets. Opens with any of these keys and its PIN.",
    location: "Location",
    opens-with: "Opens with",
    recipient: "age recipient",
    if-trouble: "If something goes wrong",
    trouble-app: "First open Kenaz and pick \"Forgot PIN\" or \"Replace a lost key\". If the app isn't available:",
    trouble-piv-pin: "Certificates PIN blocked (uses the PUK):",
    trouble-pgp-pin: "OpenPGP PIN blocked (uses the reset code):",
    trouble-mgmt: "Change certificates with another tool: ykman asks for the management key when needed. Type it at the prompt, never in the command. For example:",
    trouble-fido: "Passkey PIN blocked: it can't be unblocked. That part of the key has to be reset, which erases all its passkeys.",
  ),
)

#let t(key) = strings.at(lang).at(key)

// ---------- Style ----------

#let mono = ("DejaVu Sans Mono", "Noto Sans Mono", "Liberation Mono")
#let muted = luma(90)
#let rule = luma(150)

#set document(
  title: t("title") + " · " + data.key.label + " · " + str(data.key.serial),
)
#set text(font: ("Noto Sans", "DejaVu Sans"), size: 9pt, lang: lang)
#set par(justify: false, leading: 0.55em)
#set table(stroke: 0.5pt + rule, inset: (x: 5pt, y: 4pt))

#set page(
  paper: data.at("paper", default: "us-letter"),
  margin: (x: 16mm, top: 16mm, bottom: 18mm),
  footer: context [
    #set text(size: 7.5pt, fill: muted)
    #t("title") · #data.key.label · #t("serial") #data.key.serial · #t("sheet-id") #data.sheet.id
    #h(1fr)
    #t("page") #counter(page).display() #t("of") #counter(page).final().first()
  ],
)

// Splits a machine-made secret into groups so it can be read aloud and
// typed back without losing place.
#let grouped(value, size: 4) = {
  value.clusters().chunks(size).map(c => c.join()).join(" ")
}

#let secret(value, size: 4) = box(
  stroke: 0.8pt,
  inset: (x: 6pt, y: 5pt),
  radius: 2pt,
  text(font: mono, size: 10.5pt, weight: "bold", grouped(value, size: size)),
)

#let code(value) = text(font: mono, size: 8pt, value)

#let label-width = 42mm

// Short sections stay on one page; sections that grow with the user's data
// (certificates, codes, passkeys) pass `long: true` and may break.
#let section(title, body, long: false) = block(breakable: long, above: 12pt, below: 4pt)[
  #block(below: 6pt, width: 100%, stroke: (bottom: 1pt), inset: (bottom: 3pt), sticky: true)[
    #text(size: 11pt, weight: "bold", title)
  ]
  #body
]

#let note(body) = text(size: 8pt, fill: muted, body)

// Empty boxes, one character each, for values written by hand.
#let cells(n) = box(table(
  columns: n * (7.5mm,),
  rows: 9.5mm,
  stroke: 0.9pt,
  ..range(n).map(_ => []),
))

#let handwritten(label, hint, n) = grid(
  columns: (label-width, auto, 1fr),
  column-gutter: 8pt,
  align: (left + horizon, left + horizon, left + horizon),
  text(weight: "bold", size: 10pt, label), cells(n), note(hint),
)

#let line-field(label) = grid(
  columns: (auto, 1fr),
  column-gutter: 6pt,
  align: bottom,
  text(weight: "bold", label), block(height: 9mm, width: 100%, stroke: (bottom: 0.9pt)),
)

#let facts(rows) = table(
  columns: (label-width, 1fr),
  stroke: none,
  inset: (x: 0pt, y: 2.5pt),
  ..rows.map(((k, v)) => (text(fill: muted, k), v)).flatten(),
)

#let has(key) = key in data and data.at(key) != none

// ---------- Header ----------

#grid(
  columns: (1fr, auto),
  align: (left + bottom, right + bottom),
  [
    #text(size: 18pt, weight: "bold", t("title")) \
    #text(size: 10pt, fill: muted, t("subtitle"))
  ],
  [
    #text(size: 12pt, weight: "bold", data.key.label) \
    #text(size: 8.5pt, fill: muted)[
      #t("generated") #data.sheet.printed · #t("sheet-id") #code(data.sheet.id)
    ]
  ],
)

#block(
  above: 10pt,
  width: 100%,
  stroke: 1.5pt,
  inset: 8pt,
  radius: 3pt,
)[
  #text(weight: "bold", size: 10pt, t("warning-title")) \
  #t("warning")
]

// ---------- Written by hand ----------

#let piv-slots = if has("piv") { data.piv.at("slots", default: ()) } else { () }

#section(t("by-hand"))[
  #note(t("by-hand-note"))
  #v(4pt)
  #handwritten(t("pin"), t("pin-note"), 8)
  #v(4pt)
  #if has("piv") {
    handwritten(t("puk"), t("puk-note"), 8)
    v(4pt)
  }
  #if has("openpgp") {
    handwritten(t("admin-pin"), t("admin-pin-note"), 8)
    v(4pt)
    handwritten(t("reset-code"), t("reset-code-note"), 8)
    v(4pt)
  }
  #if has("secrets_file") {
    line-field(t("copy-location"))
  }
]

// ---------- Device ----------

#section(t("device"))[
  #facts((
    (t("label"), data.key.label),
    (t("role"), if data.key.role == "spare" { t("role-spare") } else { t("role-main") }),
    (t("model"), data.key.model),
    (t("serial"), code(str(data.key.serial))),
    (t("firmware"), code(data.key.firmware)),
    (t("form-factor"), data.key.form_factor),
    (t("nfc"), if data.key.nfc { t("yes") } else { t("no") }),
  ))
]

// ---------- PIV ----------

#let origin-label(o) = (
  generated: t("origin-generated"),
  imported: t("origin-imported"),
  efirma: t("origin-efirma"),
).at(o)

#if has("piv") {
  section(t("piv"), long: true)[
    #let mk = data.piv.management_key
    #grid(
      columns: (label-width, 1fr),
      align: (left + horizon, left + horizon),
      row-gutter: 4pt,
      text(weight: "bold", t("management-key")), secret(mk.value),
      text(fill: muted, t("algorithm")), code(mk.algorithm),
    )
    #note(t("management-key-note"))

    #for s in piv-slots {
      block(breakable: false, above: 8pt, width: 100%, stroke: 0.5pt + rule, inset: 6pt, radius: 2pt)[
        #text(weight: "bold")[#t("slot") #s.slot · #s.purpose]
        #h(1fr)
        #text(size: 8pt, fill: muted, origin-label(s.origin))
        #v(-2pt)
        #facts((
          (t("key-type"), code(s.key_type)),
          (t("subject"), s.subject),
          (t("issuer"), s.issuer),
          (t("cert-serial"), code(s.serial)),
          (t("fingerprint"), code(grouped(s.fingerprint))),
          (t("valid"), [#code(s.not_before) #t("to") #code(s.not_after)]),
        ))
      ]
    }
  ]
}

// ---------- OpenPGP ----------

#if has("openpgp") {
  let pgp = data.openpgp
  section(t("openpgp"))[
    #facts((
      (t("cardholder"), pgp.name),
      (t("email"), pgp.email),
      (t("touch"), if pgp.touch { t("yes") } else { t("no") }),
    ))
    #table(
      columns: (auto, auto, 1fr),
      table.header(
        text(weight: "bold", t("purpose")),
        text(weight: "bold", t("key-type")),
        text(weight: "bold", t("fingerprint")),
      ),
      ..pgp.keys.map(k => (
        t("key-" + k.usage),
        code(k.algorithm),
        code(grouped(k.fingerprint)),
      )).flatten(),
    )
  ]
}

// ---------- 6-digit codes ----------

#if has("oath") and data.oath.len() > 0 {
  section(t("codes"), long: true)[
    #note(t("codes-note"))
    #for a in data.oath {
      block(breakable: false, above: 6pt, width: 100%, stroke: 0.5pt + rule, inset: 6pt, radius: 2pt)[
        #grid(
          columns: if "qr_svg" in a { (1fr, 26mm) } else { (1fr,) },
          column-gutter: 8pt,
          [
            #text(weight: "bold", a.issuer) · #a.account \
            #v(2pt)
            #secret(a.secret) \
            #v(2pt)
            #note[
              #t("settings"): #a.algorithm · #a.digits · #a.period s
              #if a.touch [· #t("touch")]
            ]
          ],
          ..if "qr_svg" in a {
            (image(bytes(a.qr_svg), format: "svg", width: 26mm),)
          } else { () },
        )
      ]
    }
  ]
}

// ---------- OTP slots ----------

#let otp-slot(s) = {
  if s == none or s.type == "empty" {
    t("otp-empty")
  } else if s.type == "yubico" {
    t("otp-yubico")
  } else if s.type == "challenge-response" [
    #t("otp-chalresp")
    #if s.touch [· #t("touch")]
    · #t("used-for"): #s.used_for \
    #v(2pt)
    #secret(s.secret)
  ]
}

#if has("otp") {
  section(t("otp"))[
    #facts((
      (t("slot-short"), otp-slot(data.otp.at("short", default: none))),
      (t("slot-long"), otp-slot(data.otp.at("long", default: none))),
    ))
  ]
}

// ---------- Passkeys ----------

#if has("passkeys") and data.passkeys.len() > 0 {
  section(t("passkeys"), long: true)[
    #note(t("passkeys-note"))
    #table(
      columns: (1fr, 1fr),
      table.header(text(weight: "bold", t("site")), text(weight: "bold", t("user"))),
      ..data.passkeys.map(p => (p.site, p.user)).flatten(),
    )
  ]
}

// ---------- Secrets file ----------

#if has("secrets_file") {
  let sf = data.secrets_file
  section(t("secrets-file"))[
    #note(t("secrets-file-note"))
    #v(2pt)
    #facts(((t("location"), code(sf.path)),))
    #table(
      columns: (auto, 1fr),
      table.header(text(weight: "bold", t("opens-with")), text(weight: "bold", t("recipient"))),
      ..sf.unlock.map(u => (u.method, code(u.recipient))).flatten(),
    )
  ]
}

// ---------- If something goes wrong ----------

#section(t("if-trouble"))[
  #t("trouble-app")
  #set list(spacing: 6pt)
  #if has("piv") [
    - #t("trouble-piv-pin") #code("ykman piv access unblock-pin")
    - #t("trouble-mgmt") #code("ykman piv certificates import 9c certificado.cer")
  ]
  #if has("openpgp") [
    - #t("trouble-pgp-pin") #code("ykman openpgp access unblock-pin")
  ]
  #if has("passkeys") [
    - #t("trouble-fido")
  ]
]
