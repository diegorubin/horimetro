#ifndef __HORIMETRO_SERVICES_H__
#define __HORIMETRO_SERVICES_H__

#include <gio/gio.h>

static const gchar introspection_xml[] =
  "<node>"
  "  <interface name='com.diegorubin.horimetro'>"
  "    <method name='AddLastCommand'>"
  "      <arg type='s' name='command' direction='in'/>"
  "    </method>"
  "  </interface>"
  "</node>";

void on_bus_acquired(GDBusConnection *connection,
                     const gchar *name,
                     gpointer user_data);

void on_name_acquired(GDBusConnection *connection,
                      const gchar *name,
                      gpointer user_data);

void on_name_lost(GDBusConnection *connection,
                  const gchar *name,
                  gpointer user_data);

#endif
