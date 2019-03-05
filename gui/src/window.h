#ifndef __HORIMETRO_WINDOW_H__
#define __HORIMETRO_WINDOW_H__

#include <gtk/gtk.h>

#include "main_display_frame.h"
#include "last_command_frame.h"

void main_window_activate(GtkApplication* app, gpointer user_data);
void hide_frames();
void show_next_frame();

#endif

