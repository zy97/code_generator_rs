using Prism.Mvvm;

namespace CodeGeneratorApp.ViewModels
{
    public class MainWindowViewModel : BindableBase
    {
        private string _title = "Abp 代码生成器";
        public string Title
        {
            get { return _title; }
            set { SetProperty(ref _title, value); }
        }

        public MainWindowViewModel()
        {

        }
    }
}
