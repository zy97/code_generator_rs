using Prism.Mvvm;
using ReactiveUI.Fody.Helpers;

namespace CodeGeneratorApp.Modules.ReactGenerate.ViewModels
{
    public class ViewAViewModel : BindableBase
    {
        private string _message;
        public string Message
        {
            get { return _message; }
            set { SetProperty(ref _message, value); }
        }
        [Reactive]
        public string Title { get; set; } = "Abp React生成";
        public ViewAViewModel()
        {
            Message = "View A from your Prism Module";
        }
    }
}
