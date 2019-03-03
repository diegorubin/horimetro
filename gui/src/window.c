#include "window.h"

void main_window_activate(GtkApplication* app, gpointer user_data)
{
  GtkWidget *window;
  GtkWidget *main_box;
  GtkWidget *command_frame;

  window = gtk_application_window_new(app);
  gtk_window_set_title(GTK_WINDOW(window), "Horimetro");
  gtk_window_set_default_size(GTK_WINDOW(window), 200, 200);
  gtk_window_fullscreen(GTK_WINDOW(window));
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);

  main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  
  gtk_container_add(GTK_CONTAINER(window), main_box);

  /* add last command frame */
  command_frame = build_last_command_frame();
  gtk_box_pack_start(GTK_BOX(main_box), command_frame, TRUE, TRUE, 0);

  gtk_widget_show_all(window);
}

