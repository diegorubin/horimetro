AC_INIT([horimetro],[1.0.0],[rubin.diego@gmail.com])

AM_INIT_AUTOMAKE(horimentro,1.0.0)

AC_PROG_CC

AC_PROG_INSTALL

AC_CONFIG_SRCDIR([src])

PKG_CHECK_MODULES([horimetro],[gtk+-3.0])

dnl Add gsettings rules
GLIB_GSETTINGS

AC_CONFIG_FILES([
Makefile
src/Makefile
])

AC_OUTPUT(horimetro.desktop)

