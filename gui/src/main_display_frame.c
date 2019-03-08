#include "main_display_frame.h"

guint check_in = 0;
guint check_out = 0;

GtkWidget *timer_label_init;
GtkWidget *timer_label_end;

GtkWidget* build_main_display_frame()
{
  GtkWidget *main_box;
  GtkWidget *timer_box;
  GtkWidget *timer_progress;

  GtkWidget *timer_label_init_description;
  GtkWidget *timer_label_end_description;

  main_display_frame = gtk_frame_new("Tarefas");

  main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  gtk_container_add(GTK_CONTAINER(main_display_frame), main_box);
  gtk_container_set_border_width(GTK_CONTAINER(main_display_frame), 0);

  timer_progress = gtk_progress_bar_new();
  gtk_box_pack_start(GTK_BOX(main_box), timer_progress, FALSE, FALSE, 0);
  gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(timer_progress), 0.5);

  timer_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
  gtk_box_pack_start(GTK_BOX(main_box), timer_box, FALSE, FALSE, 0);

  timer_label_init_description = gtk_label_new("Entrada: ");
  gtk_box_pack_start(GTK_BOX(timer_box), timer_label_init_description, FALSE, TRUE, 0);

  timer_label_init = gtk_label_new("08:00");
  gtk_box_pack_start(GTK_BOX(timer_box), timer_label_init, FALSE, TRUE, 0);

  timer_label_end = gtk_label_new("17:00");
  gtk_box_pack_end(GTK_BOX(timer_box), timer_label_end, FALSE, TRUE, 0);

  timer_label_end_description = gtk_label_new("Saída:");
  gtk_box_pack_end(GTK_BOX(timer_box), timer_label_end_description, FALSE, TRUE, 0);


  return main_display_frame;
}

void set_check_in(guint value)
{
  char *check_in_markup;
  char *check_out_markup;
  const gchar* format = "%02d:%02d";

  check_in = value;

  int init_minutes = value % 60;
  int init_hours = value / 60;

  check_in_markup = g_markup_printf_escaped(format, init_hours, init_minutes);

  check_out = 9 * 60 + value;
  int end_minutes = check_out % 60;
  int end_hours = check_out / 60;

  check_out_markup = g_markup_printf_escaped(format, end_hours, end_minutes);

  gtk_label_set_markup(GTK_LABEL(timer_label_init), check_in_markup);
  gtk_label_set_markup(GTK_LABEL(timer_label_end), check_out_markup);

  g_free(check_in_markup);
  g_free(check_out_markup);
}
