#ifndef __HORIMETRO_LAST_COMMAND_FRAME_H__
#define __HORIMETRO_LAST_COMMAND_FRAME_H__

#include <gtk/gtk.h>

GtkWidget *last_command_frame;

GtkWidget* build_last_command_frame();
void add_command(const gchar *str);

#endif
