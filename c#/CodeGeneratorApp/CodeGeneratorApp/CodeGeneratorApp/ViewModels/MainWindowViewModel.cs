using CodeGeneratorApp.Core;
using CodeGeneratorApp.Core.Mvvm;
using Prism.Regions;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using System.Reactive;

namespace CodeGeneratorApp.ViewModels
{
    public class MainWindowViewModel : ViewModelBase
    {
        private readonly IRegionManager regionManager;

        [Reactive]
        public string Title { get; set; } = "Abp 代码生成器";
        public ReactiveCommand<Unit, Unit> LoadAllViews { get; set; }


        public MainWindowViewModel(IRegionManager regionManager)
        {
            LoadAllViews = ReactiveCommand.Create(() =>
            {
                regionManager.AddToRegion(RegionNames.ContentRegion, "ViewA");
                regionManager.AddToRegion(RegionNames.ContentRegion, "ViewB");
            });
        }
    }
}
