AC_INIT([horimetro],[1.0.0],[rubin.diego@gmail.com])

AM_INIT_AUTOMAKE(horimentro,1.0.0)

AC_PROG_CC

AC_PROG_INSTALL

AC_CONFIG_SRCDIR([src])

PKG_CHECK_MODULES([horimetro],[gtk+-3.0])

IT_PROG_INTLTOOL([0.35.0])

GETTEXT_PACKAGE=horimetro
AC_SUBST(GETTEXT_PACKAGE)
AC_DEFINE_UNQUOTED([GETTEXT_PACKAGE], ["$GETTEXT_PACKAGE"], [Gettext domain])

AM_GLIB_GNU_GETTEXT

dnl Add gsettings rules
GLIB_GSETTINGS

HORIMETRO_LOCALEDIR=[${datadir}/locale]
AC_SUBST(HORIMETRO_LOCALEDIR)

AC_CONFIG_FILES([
Makefile
src/Makefile
])

AC_OUTPUT(horimetro.desktop)

