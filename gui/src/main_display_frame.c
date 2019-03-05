#include "main_display_frame.h"

GtkWidget* build_main_display_frame()
{
  main_display_frame = gtk_frame_new("Tarefas");
  return main_display_frame;
}
