#include "last_command_frame.h"

GtkWidget *commands_box;

GtkWidget* build_last_command_frame()
{
  GtkWidget *scrolled_window = gtk_scrolled_window_new(NULL, NULL);

  last_command_frame = gtk_frame_new("Últimos Comandos");
  commands_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);

  gtk_container_add(GTK_CONTAINER(scrolled_window), commands_box);
  gtk_container_add(GTK_CONTAINER(last_command_frame), scrolled_window);

  return last_command_frame;
}

void add_command(const gchar *str)
{
  GtkWidget *label = gtk_label_new(str);
  gtk_widget_set_halign(label, GTK_ALIGN_START);
  gtk_box_pack_end(GTK_BOX(commands_box), label, FALSE, FALSE, 0);
  gtk_widget_show(GTK_WIDGET(label));
}
