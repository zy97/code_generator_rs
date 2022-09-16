using CodeGeneratorApp.Modules.ModuleName.ViewModels;
using Microsoft.Win32;
using System.Windows.Controls;

namespace CodeGeneratorApp.Modules.ModuleName.Views
{
    /// <summary>
    /// Interaction logic for ViewA.xaml
    /// </summary>
    public partial class ViewA : UserControl
    {
        public ViewA()
        {
            InitializeComponent();
            var viewModel = this.DataContext as ViewAViewModel;
            viewModel.SelectEntityInteraction.RegisterHandler(interact =>
            {
                var openFileDialog = new OpenFileDialog()
                {
                    Filter = "C# 文件(.cs)|*.cs"
                };
                if (openFileDialog.ShowDialog() is true)
                {
                    interact.SetOutput(openFileDialog.FileName);
                    return;
                }
            });
        }
    }
}
