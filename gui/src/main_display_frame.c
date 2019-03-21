#include "main_display_frame.h"

guint check_in = 0;
guint check_out = 0;
gfloat elapsed = 0;

GtkWidget *timer_label_init;
GtkWidget *timer_label_end;
GtkWidget *timer_progress;
GtkWidget *timer_label_elapsed;

GtkWidget *task_label_code;
GtkWidget *task_label_description;
GtkWidget *last_task_label_description;

GtkWidget* build_main_display_frame()
{
  GtkWidget *main_box;
  GtkWidget *timer_box;
  GtkWidget *tasks_box;
  GtkWidget *current_tasks_box;

  GtkWidget *timer_label_init_description;
  GtkWidget *timer_label_end_description;

  main_display_frame = gtk_frame_new("Tarefas");

  main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  gtk_container_add(GTK_CONTAINER(main_display_frame), main_box);
  gtk_container_set_border_width(GTK_CONTAINER(main_display_frame), 0);

  timer_progress = gtk_progress_bar_new();
  gtk_box_pack_start(GTK_BOX(main_box), timer_progress, FALSE, FALSE, 0);
  gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(timer_progress), 0);

  timer_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
  gtk_box_pack_start(GTK_BOX(main_box), timer_box, FALSE, FALSE, 0);

  timer_label_init_description = gtk_label_new("");
  gtk_label_set_markup(GTK_LABEL(timer_label_init_description), "<span font=\"10\"><b>Entrada: </b></span>");
  gtk_box_pack_start(GTK_BOX(timer_box), timer_label_init_description, FALSE, TRUE, 0);

  timer_label_init = gtk_label_new("");
  gtk_label_set_markup(GTK_LABEL(timer_label_init), "");
  gtk_box_pack_start(GTK_BOX(timer_box), timer_label_init, FALSE, TRUE, 0);

  timer_label_end = gtk_label_new("");
  gtk_label_set_markup(GTK_LABEL(timer_label_end), "");
  gtk_box_pack_end(GTK_BOX(timer_box), timer_label_end, FALSE, TRUE, 0);

  timer_label_end_description = gtk_label_new("");
  gtk_label_set_markup(GTK_LABEL(timer_label_end_description), "<span font=\"10\"><b>Saída: </b></span>");
  gtk_box_pack_end(GTK_BOX(timer_box), timer_label_end_description, FALSE, TRUE, 0);

  tasks_box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
  gtk_box_pack_start(GTK_BOX(main_box), tasks_box, TRUE, TRUE, 0);

  timer_label_elapsed = gtk_label_new("");
  gtk_label_set_markup(GTK_LABEL(timer_label_elapsed), "<span font=\"50\"><b>00:00</b></span>");
  gtk_box_pack_start(GTK_BOX(tasks_box), timer_label_elapsed, FALSE, TRUE, 0);

  current_tasks_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  gtk_box_pack_start(GTK_BOX(tasks_box), current_tasks_box, TRUE, TRUE, 0);

  task_label_code = gtk_label_new("");
  gtk_box_pack_start(GTK_BOX(current_tasks_box), task_label_code, TRUE, TRUE, 0);

  task_label_description = gtk_label_new("");
  gtk_label_set_line_wrap(GTK_LABEL(task_label_description), TRUE);
  gtk_box_pack_start(GTK_BOX(current_tasks_box), task_label_description, TRUE, TRUE, 0);

  last_task_label_description = gtk_label_new("");
  gtk_label_set_line_wrap(GTK_LABEL(last_task_label_description), TRUE);
  gtk_box_pack_start(GTK_BOX(current_tasks_box), last_task_label_description, TRUE, TRUE, 10);

  return main_display_frame;
}

void set_current_task(const gchar* code, const gchar* description)
{
  char *last_task_markup;
  const gchar* last_task_format = "%s - %s";
  last_task_markup = g_markup_printf_escaped(last_task_format, 
    gtk_label_get_text(GTK_LABEL(task_label_code)),
    gtk_label_get_text(GTK_LABEL(task_label_description)));

  if (g_strcmp0(last_task_markup, " - ")) {
    gtk_label_set_markup(GTK_LABEL(last_task_label_description), last_task_markup);
  }

  char *code_markup;
  const gchar* code_format = "<span font=\"20\"><b>%s</b></span>";
  code_markup = g_markup_printf_escaped(code_format, code);
  gtk_label_set_markup(GTK_LABEL(task_label_code), code_markup);

  char *description_markup;
  const gchar* description_format = "<span font=\"10\"><b>%s</b></span>";
  description_markup = g_markup_printf_escaped(description_format, description);
  gtk_label_set_markup(GTK_LABEL(task_label_description), description_markup);

  g_free(last_task_markup);
  g_free(code_markup);
  g_free(description_markup);
}

gboolean update_elapsed_time(gpointer data)
{
  char *elapsed_markup;

  elapsed = elapsed + 1;
  gfloat value = elapsed / (check_out - check_in);

  if (value <= 1) {
    int elapsed_minutes = ((int) ((check_out - check_in) - elapsed)) % 60;
    int elapsed_hours = ((int) (((check_out - check_in) - elapsed))) / 60;

    const gchar* format = "<span color=\"red\" font=\"50\"><b>%02d:%02d</b></span>";
    elapsed_markup = g_markup_printf_escaped(format, elapsed_hours, elapsed_minutes);

    gtk_progress_bar_set_fraction(GTK_PROGRESS_BAR(timer_progress), value);
  } else {
    int elapsed_minutes = ((int) (elapsed - (check_out - check_in)))  % 60;
    int elapsed_hours = ((int) (elapsed - (check_out - check_in))) / 60;

    const gchar* format = "<span color=\"green\" font=\"50\"><b>%02d:%02d</b></span>";
    elapsed_markup = g_markup_printf_escaped(format, elapsed_hours, elapsed_minutes);
  }

  gtk_label_set_markup(GTK_LABEL(timer_label_elapsed), elapsed_markup);
  g_free(elapsed_markup);

  return TRUE;
}

void set_check_in(guint value, guint elapsed_in_check_in)
{
  char *check_in_markup;
  char *check_out_markup;
  const gchar* format = "<span font=\"10\"><b>%02d:%02d</b></span>";

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
  
  elapsed = elapsed_in_check_in - 1;

  update_elapsed_time(NULL);
  g_timeout_add(60000, update_elapsed_time, NULL);

  g_free(check_in_markup);
  g_free(check_out_markup);
}
