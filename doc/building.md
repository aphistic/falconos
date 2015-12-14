Troubleshooting
---------------

**ISO won't build**

`grub-mkrescue: warning: Your xorriso doesn't support '--grub2-boot-info'. Some features are disabled. Please use xorriso 1.2.9 or later..`

If you see this error, try installing the `libisoburn` package.

`xorriso : FAILURE : Cannot find path '/efi.img' in loaded ISO image`

If you see this error, try installing the `mtools` package.