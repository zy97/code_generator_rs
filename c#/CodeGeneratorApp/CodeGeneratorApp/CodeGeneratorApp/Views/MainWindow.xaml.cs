using CodeGeneratorApp.ViewModels;
using System;

using System.Reactive;
using System.Windows;

namespace CodeGeneratorApp.Views
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            this.Loaded += MainWindow_Loaded;
        }
        private void MainWindow_Loaded(object sender, RoutedEventArgs e)
        {
            var vm = this.DataContext as MainWindowViewModel;
            vm.LoadAllViews.Execute(Unit.Default).Subscribe();
        }
    }
}
