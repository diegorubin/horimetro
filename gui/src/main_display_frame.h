#ifndef __HORIMETRO_MAIN_DISPLAY_FRAME_H__
#define __HORIMETRO_MAIN_DISPLAY_FRAME_H__

#include <gtk/gtk.h>

GtkWidget *main_display_frame;

GtkWidget* build_main_display_frame();

void set_check_in(guint value, guint elapsed_in_check_in, guint fixed_checkout);
void set_current_task(const gchar* code, const gchar* description);

#endif
