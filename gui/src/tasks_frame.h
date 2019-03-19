#ifndef __HORIMETRO_TASKS_FRAME_H__
#define __HORIMETRO_TASKS_FRAME_H__

#include <gtk/gtk.h>

GtkWidget *tasks_frame;

GtkWidget* build_tasks_frame();
void add_task(const gchar *date, const gchar *description, 
  const gchar *init, const gchar *total);
void clear_tasks();

#endif
