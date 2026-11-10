# Draft glossary / Glosario provisional

Plain terms are for the main path; advanced terms explain them, not interchangeable labels. Translation and usability review remain pending.

| English: plain | Español: lenguaje claro | Advanced term and distinction |
|---|---|---|
| Your YubiKey | Tu YubiKey | Physical authenticator / autenticador físico; its available features depend on model. |
| Your spare key | Tu llave de repuesto | Backup authenticator / autenticador de respaldo. Register it separately at each site; a website credential is not copied from the first YubiKey. |
| Use your key to sign in | Usa tu llave para iniciar sesión | Passkey stored on a roaming authenticator / llave de acceso en un autenticador externo. A platform or synced passkey can instead live on a device or in a password manager. |
| Use your key after your password | Usa tu llave después de la contraseña | FIDO security key as a second factor / llave de seguridad FIDO como segundo factor. It is not necessarily a discoverable passkey. |
| Six-digit code | Código de seis dígitos | OATH TOTP / contraseña de un solo uso basada en el tiempo. Both compatible keys need the same setup secret; it cannot be read back from the first key. |
| Setup QR or setup link | QR o enlace de configuración | TOTP provisioning URI / URI de aprovisionamiento TOTP. It contains a secret: keep it private. |
| Backup codes from the site | Códigos de respaldo del sitio | Provider recovery codes / códigos de recuperación del proveedor. These are not the changing six-digit codes; store them privately outside the app. Some providers issue one account recovery code instead. |
| Your PIN | Tu PIN | FIDO, OpenPGP user and PIV PINs are separate applet PINs. The product intends to use the same value, but changing one does not automatically change the others; when they are set remains open (D8). |
| Your secrets file | Tu archivo de secretos | Encrypted age file / archivo cifrado age. It can hold setup secrets and imported keys, not restore website passkeys or security-key registrations. |
| Recovery sheet | Hoja de recuperación | Printed recovery record / hoja impresa de recuperación. PINs, PUK and OpenPGP admin/reset codes are handwritten, not printed; private keys are not printed. |
| Sign or encrypt email and files | Firma o cifra correos y archivos | OpenPGP signing and encryption / firma y cifrado OpenPGP. The app guides external tools; it does not read or send email. |
| Sign in to your computer / certificates | Inicia sesión en tu computadora / certificados | PIV smart card and X.509 certificate / tarjeta inteligente PIV y certificado X.509. Availability depends on the key and platform. |
| Touch to use | Toca para usar | Touch policy / política de toque. A physical confirmation may be required for an operation. |
| Reset the passkey part | Restablece la parte de llaves de acceso | FIDO reset / restablecimiento FIDO. Invalidates that key's FIDO credentials, including second-factor registrations, and can remove its FIDO secrets-file unlock; it is not a reset of every applet. |
