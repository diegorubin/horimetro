#include "services.h"

extern GDBusNodeInfo *introspection_data;

void handle_method_call(GDBusConnection *connection,
                        const gchar *sender,
                        const gchar *object_path,
                        const gchar *interface_name,
                        const gchar *method_name,
                        GVariant *parameters,
                        GDBusMethodInvocation *invocation,
                        gpointer user_data)
{
  if (!g_strcmp0 (method_name, "AddLastCommand")) {
    
  }
}

static const GDBusInterfaceVTable interface_vtable =
{
  handle_method_call,
  NULL,
  NULL,
};

void on_bus_acquired(GDBusConnection *connection,
                     const gchar *name,
                     gpointer user_data)
{
  guint registration_id;

  registration_id = g_dbus_connection_register_object(connection,
                                                      "/com/diegorubin/horimetro",
                                                      introspection_data->interfaces[0],
                                                      &interface_vtable,
                                                      NULL,  /* user_data */
                                                      NULL,  /* user_data_free_func */
                                                      NULL); /* GError** */
  g_assert (registration_id > 0);

}

void on_name_acquired(GDBusConnection *connection,
                      const gchar *name,
                      gpointer user_data)
{
}

void on_name_lost(GDBusConnection *connection,
                  const gchar *name,
                  gpointer user_data)
{
  exit (1);
}

