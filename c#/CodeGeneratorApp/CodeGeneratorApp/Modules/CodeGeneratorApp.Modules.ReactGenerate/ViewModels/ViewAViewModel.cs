using Prism.Mvvm;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using System.Reactive;

namespace CodeGeneratorApp.Modules.ReactGenerate.ViewModels
{

    public class ViewAViewModel : BindableBase
    {
        [Reactive]
        public string Title { get; set; } = "Abp React生成";

        #region 生成代码相关

        [Reactive]
        public string EntityPath { get; set; }
        [Reactive]
        public bool CreateApi { get; set; }
        [Reactive]
        public bool ApiPrefix { get; set; }
        [Reactive]
        public bool CreateStore { get; set; }
        [Reactive]
        public bool CreatePage { get; set; }
        [Reactive]
        public bool Format { get; set; }
        public Interaction<Unit, string> SelectEntityInteraction { get; } = new Interaction<Unit, string>();
        public ReactiveCommand<Unit, Unit> SelectEntity { get; set; }
        public ReactiveCommand<Unit, Unit> Generate { get; set; }

        #endregion

        public ViewAViewModel()
        {
        }
    }
}
