#include <gtk/gtk.h>
#include <glib.h>
#include <gio/gio.h>

#include "tasks_frame.h"
#include "last_command_frame.h"
#include "window.h"

#define DAEMON_NAME     "com.diegorubin.horimetro"
#define DAEMON_PATH     "/com/diegorubin/horimetro"
#define DAEMON_INTERFACE  "com.diegorubin.horimetro.Gui"

static const gchar introspection_xml[] = 
"<node>"                                  
"  <interface name='com.diegorubin.horimetro.Gui'>"              
"    <method name='AddLastCommand'>"                      
"      <arg type='s' name='command' direction='in'/>"              
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"    <method name='AddTask'>"                      
"      <arg type='s' name='date' direction='in'/>"              
"      <arg type='s' name='description' direction='in'/>"              
"      <arg type='s' name='init' direction='in'/>"              
"      <arg type='s' name='total' direction='in'/>"              
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"    <method name='ClearTasks'>"                      
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"    <method name='SetCurrentTask'>"                      
"      <arg type='s' name='code' direction='in'/>"              
"      <arg type='s' name='description' direction='in'/>"              
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"    <method name='ShowNextFrame'>"                      
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"    <method name='CheckIn'>"                      
"      <arg type='u' name='value' direction='in'/>"              
"      <arg type='u' name='elapsed' direction='in'/>"              
"      <arg type='u' name='fixed_checkout' direction='in'/>"              
"      <arg type='s' name='result' direction='out'/>"              
"    </method>"                               
"  </interface>"                              
"</node>"; 

void method_handler(GDBusConnection *conn,
        const gchar *sender,
        const gchar *object_path,
        const gchar *interface_name,
        const gchar *method_name,
        GVariant *parameters,
        GDBusMethodInvocation *invocation,
        gpointer user_data)
{
  if (!g_strcmp0(method_name, "AddLastCommand")) {
    const gchar* command;

    g_variant_get(parameters, "(s)", &command);

    add_command(command);

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", command));
    return;
  }

  if (!g_strcmp0(method_name, "AddTask")) {
    const gchar* date;
    const gchar* description;
    const gchar* init;
    const gchar* total;

    g_variant_get(parameters, "(ssss)", &date, &description, &init, &total);

    add_task(date, description, init, total);

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", "ok"));
    return;
  }

  if (!g_strcmp0(method_name, "SetCurrentTask")) {
    const gchar* code;
    const gchar* description;

    g_variant_get(parameters, "(ss)", &code, &description);

    set_current_task(code, description);

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", "ok"));
    return;
  }

  if (!g_strcmp0(method_name, "ShowNextFrame")) {
    show_next_frame();

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", "ok"));
    return;
  }

  if (!g_strcmp0(method_name, "ClearTasks")) {
    clear_tasks();

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", "ok"));
    return;
  }

  if (!g_strcmp0(method_name, "CheckIn")) {
    const guint value;
    const guint elapsed;
    const guint fixed_checkout;

    g_variant_get(parameters, "(uuu)", &value, &elapsed, &fixed_checkout);

    set_check_in(value, elapsed, fixed_checkout);

    g_dbus_method_invocation_return_value(invocation,
                    g_variant_new("(s)", "ok"));
    return;
  }

  g_dbus_method_invocation_return_error(invocation,
      G_DBUS_ERROR,
      G_DBUS_ERROR_INVALID_ARGS,
      "Invalid method");
}

struct appcore {
  GDBusConnection *conn;
  guint owner_id;
  GDBusNodeInfo *node_info;

  GMainLoop *loop;
};

static const GDBusInterfaceVTable vtable = {
  method_handler,
  NULL,
  NULL,
};

static void on_bus_acquired(GDBusConnection *conn,
            const gchar *name,
            gpointer user_data)
{
  guint regid;
  GError *err;

  struct appcore *data = (struct appcore *)user_data;
  data->conn = conn;

  err = NULL;
  regid = g_dbus_connection_register_object(conn,
                     DAEMON_PATH,
                     data->node_info->interfaces[0],
                     &vtable,
                     user_data,
                     NULL,
                     &err);
  if (!regid) {
    g_print("g_dbus_connection_register_object() is failed: %s", err->message);
    g_error_free(err);
  }
}

static void on_name_acquired(GDBusConnection *conn,
            const gchar *name,
            gpointer user_data)
{
}

static void on_name_lost(GDBusConnection *conn,
            const gchar *name,
            gpointer user_data)
{
  g_print("on_name_lost() is called");
}


static int init_dbus(struct appcore *data)
{
  GError *err = NULL;
  data->node_info = g_dbus_node_info_new_for_xml(introspection_xml, &err);
  if (!data->node_info) {
    g_print("g_dbus_node_info_new_for_xml() is failed: %s", err->message);
    g_error_free(err);
    return -1;
  }

  data->owner_id = g_bus_own_name(G_BUS_TYPE_SESSION,
                  DAEMON_NAME,
                  G_BUS_NAME_OWNER_FLAGS_NONE,
                  on_bus_acquired,
                  on_name_acquired,
                  on_name_lost,
                  data,
                  NULL);
  if (!data->owner_id) {
    g_print("g_bus_own_name() is failed");
    return -1;
  }
  return 0;
}

int main (int argc, char **argv)
{

  struct appcore data = {
    .conn = NULL,
    .node_info = NULL,
  };

  GtkApplication *app;
  int status;

  app = gtk_application_new("com.diegorubin.horimetro.gui", G_APPLICATION_FLAGS_NONE);
  g_signal_connect(app, "activate", G_CALLBACK (main_window_activate), NULL);

  init_dbus(&data);

  status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref (app);

  return status;
}
