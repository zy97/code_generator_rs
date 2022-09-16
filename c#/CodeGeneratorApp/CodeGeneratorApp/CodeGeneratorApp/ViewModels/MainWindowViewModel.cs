using CodeGeneratorApp.Core.Mvvm;
using Prism.Regions;
using ReactiveUI.Fody.Helpers;

namespace CodeGeneratorApp.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        private readonly IRegionManager regionManager;

        [Reactive]
        public string Title { get; set; } = "Abp 代码生成器";


        public MainWindowViewModel(IRegionManager regionManager)
        {
        }
    }
}
