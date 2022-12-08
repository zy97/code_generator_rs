using System.Windows;
using System.Windows.Controls;
using CodeGeneratorApp.Modules.ModuleName.ViewModels;

namespace CodeGeneratorApp.Modules.ModuleName.Views
{
    /// <summary>
    /// Interaction logic for ViewA.xaml
    /// </summary>
    public partial class ViewA : UserControl
    {
        private readonly ViewAViewModel viewModel;

        public ViewA()
        {
            InitializeComponent();
            viewModel = this.DataContext as ViewAViewModel;
        }
        private void EntityFilePreviewDrop(object sender, DragEventArgs e)
        {
            var f = e.Data.GetData(DataFormats.FileDrop) as string[];
            if (f?.Length > 0)
            {
                this.viewModel.EntityPath = f[0];
            }
        }

        private void EntityFilePreviewDragOver(object sender, DragEventArgs e)
        {
            e.Effects = DragDropEffects.Copy;
            e.Handled = true;
        }
    }
}
