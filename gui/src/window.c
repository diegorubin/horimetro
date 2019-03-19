#include "window.h"

#define TOTAL_FRAMES 3

GtkWidget *main_box;
GtkWidget *command_frame;
GtkWidget *tasks_frame;
GtkWidget *display_frame;

int current_frame = 0;

void main_window_activate(GtkApplication* app, gpointer user_data)
{
  GtkWidget *window;

  window = gtk_application_window_new(app);
  gtk_window_set_title(GTK_WINDOW(window), "Horimetro");
  gtk_window_set_default_size(GTK_WINDOW(window), 480, 320);
  gtk_window_fullscreen(GTK_WINDOW(window));
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);

  main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  
  gtk_container_add(GTK_CONTAINER(window), main_box);

  /* add main display frame */
  display_frame = build_main_display_frame();
  gtk_box_pack_start(GTK_BOX(main_box), display_frame, TRUE, TRUE, 0);

  /* add tasks frame */
  tasks_frame = build_tasks_frame();
  gtk_box_pack_start(GTK_BOX(main_box), tasks_frame, TRUE, TRUE, 0);

  /* add last command frame */
  command_frame = build_last_command_frame();
  gtk_box_pack_start(GTK_BOX(main_box), command_frame, TRUE, TRUE, 0);

  gtk_widget_show_all(window);

  show_next_frame();
}

void hide_frames()
{
  gtk_widget_hide(display_frame);
  gtk_widget_hide(tasks_frame);
  gtk_widget_hide(command_frame);
}

void show_next_frame()
{
  hide_frames();
  if (current_frame >= TOTAL_FRAMES) current_frame = 0;
  switch(current_frame) {
    case 0:
      gtk_widget_show(display_frame);
      break;
    case 1:
      gtk_widget_show(tasks_frame);
      break;
    case 2:
      gtk_widget_show(command_frame);
      break;
  }
  current_frame++;
}

