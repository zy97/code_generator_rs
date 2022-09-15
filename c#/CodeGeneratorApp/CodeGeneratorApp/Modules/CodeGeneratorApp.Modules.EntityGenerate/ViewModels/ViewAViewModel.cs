using CodeGeneratorApp.Core.Mvvm;
using CodeGeneratorApp.Services.Interfaces;
using Prism.Regions;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using System.Reactive;

namespace CodeGeneratorApp.Modules.ModuleName.ViewModels
{
    public class ViewAViewModel : RegionViewModelBase
    {
        [Reactive]
        public string Message { get; set; }

        public ReactiveCommand<Unit, Unit> Test { get; set; }

        public ViewAViewModel(IRegionManager regionManager, IMessageService messageService) :
            base(regionManager)
        {
            Message = messageService.GetMessage();
            Test = ReactiveCommand.Create(() =>
            {
            });

        }
        public override void OnNavigatedTo(NavigationContext navigationContext)
        {
            //do something
        }
    }
}
