#include <gtk/gtk.h>

#include "services.h"
#include "window.h"

GDBusNodeInfo *introspection_data = NULL;

int main (int argc, char **argv)
{

  GError *err = NULL;
  introspection_data = g_dbus_node_info_new_for_xml(introspection_xml, &err);

  if (introspection_data) {
    g_print("g_dbus_node_info_new_for_xml() is failed: %s", err->message);
    g_error_free(err);
    return -1;
  }

  guint owner_id = g_bus_own_name(G_BUS_TYPE_SESSION,
                                  "com.diegorubin.horimetro",
                                  G_BUS_NAME_OWNER_FLAGS_NONE,
                                  on_bus_acquired,
                                  on_name_acquired,
                                  on_name_lost,
                                  NULL,
                                  NULL);


  GtkApplication *app;
  int status;

  app = gtk_application_new("com.diegorubin.horimetro.gui", G_APPLICATION_FLAGS_NONE);
  g_signal_connect(app, "activate", G_CALLBACK (main_window_activate), NULL);
  status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref (app);

  g_bus_unown_name (owner_id);

  g_dbus_node_info_unref (introspection_data);

  return status;
}
