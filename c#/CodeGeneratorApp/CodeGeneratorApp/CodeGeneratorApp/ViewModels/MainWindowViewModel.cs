using ReactiveUI;
using ReactiveUI.Fody.Helpers;

namespace CodeGeneratorApp.ViewModels
{
    public class MainWindowViewModel : ReactiveObject
    {
        [Reactive]
        public string Title { get; set; } = "Abp 代码生成器";


        public MainWindowViewModel()
        {

        }
    }
}
