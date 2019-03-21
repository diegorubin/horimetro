#include "tasks_frame.h"

GtkWidget *tasks_list;

void build_list(GtkWidget *list)
{
  GtkCellRenderer *renderer;
  ;
  GtkListStore *store;

  renderer = gtk_cell_renderer_text_new ();

  GtkTreeViewColumn *date_column = gtk_tree_view_column_new_with_attributes("Dia",
          renderer, "text", 0, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), date_column);

  GtkTreeViewColumn *init_column = gtk_tree_view_column_new_with_attributes("Início",
          renderer, "text", 1, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), init_column);

  GtkTreeViewColumn *total_column = gtk_tree_view_column_new_with_attributes("Tempo Gasto",
          renderer, "text", 2, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), total_column);

  GtkTreeViewColumn *description_column = gtk_tree_view_column_new_with_attributes("Tarefa",
          renderer, "text", 3, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), description_column);

  store = gtk_list_store_new(4, G_TYPE_STRING, G_TYPE_STRING, G_TYPE_STRING, G_TYPE_STRING);

  gtk_tree_view_set_model(GTK_TREE_VIEW(list), 
      GTK_TREE_MODEL(store));

  g_object_unref(store);
}

void add_task(const gchar *date, const gchar *description, 
  const gchar *init, const gchar *total) 
{
  GtkListStore *store;
  GtkTreeIter iter;

  store = GTK_LIST_STORE(gtk_tree_view_get_model
      (GTK_TREE_VIEW(tasks_list)));

  gtk_list_store_append(store, &iter);
  gtk_list_store_set(store, &iter, 
    0, date, 
    1, init, 
    2, total, 
    3, description, 
    -1);
}

void clear_tasks()
{
  GtkListStore *store;
  store = GTK_LIST_STORE(gtk_tree_view_get_model
      (GTK_TREE_VIEW(tasks_list)));
  gtk_list_store_clear(store);
}

GtkWidget* build_tasks_frame()
{
  GtkWidget *scrolled_window = gtk_scrolled_window_new(NULL, NULL);
  tasks_list = gtk_tree_view_new();

  tasks_frame = gtk_frame_new("Últimas Tarefas");

  gtk_container_add(GTK_CONTAINER(scrolled_window), tasks_list);
  gtk_container_add(GTK_CONTAINER(tasks_frame), scrolled_window);

  build_list(tasks_list);

  return tasks_frame;
}
