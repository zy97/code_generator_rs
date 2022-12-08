using System;
using System.Reactive;
using System.Reactive.Linq;
using CodeGeneratorApp.Core.Mvvm;
using CodeGeneratorApp.Modules.ReactGenerate.Services;
using Prism.Regions;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;

namespace CodeGeneratorApp.Modules.ReactGenerate.ViewModels
{

    public class ViewAViewModel : RegionViewModelBase
    {
        private readonly IReactGeneratorService reactGenerateService;

        [Reactive]
        public string Title { get; set; } = "Abp React生成";

        #region 生成代码相关

        [Reactive]
        public string EntityPath { get; set; }
        [Reactive]
        public bool CreateApi { get; set; }
        [Reactive]
        public string ApiPrefix { get; set; }
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

        public ViewAViewModel(IRegionManager regionManager, IReactGeneratorService reactGenerateService) : base(regionManager)
        {
            this.reactGenerateService = reactGenerateService;
            SelectEntity = ReactiveCommand.Create(() =>
            {
                SelectEntityInteraction.Handle(Unit.Default)
                    .Subscribe(entityPath =>
                    {
                        EntityPath = entityPath;
                    }, ex => { });
            });
            Generate = ReactiveCommand.Create(() =>
            {
                this.reactGenerateService.SetEntity(EntityPath);
                if (CreateApi)
                    this.reactGenerateService.CreateApi(string.IsNullOrWhiteSpace(ApiPrefix) ? "/a/b/c" : ApiPrefix);
                if (CreateStore)
                    this.reactGenerateService.CreateStore();
                if (CreatePage)
                    this.reactGenerateService.CreatePage();
                //if (CreateManager)
                //    this.entityGenerateService.CreateManager();
                //if (CreateEfCoreRepository)
                //{
                //    this.entityGenerateService.CreateEfRepository();
                //    this.entityGenerateService.CreateRepositoryInterface();
                //}
                //if (this.CreateService)
                //{
                //    var isCustom = false;
                //    if (this.IsCustomService)
                //        isCustom = true;
                //    this.entityGenerateService.CreateIService(isCustom);
                //    this.entityGenerateService.CreateService(isCustom);
                //}
                //if (this.CreateException)
                //{
                //    this.entityGenerateService.CreateException(ExceptionName, ExceptionCode, ExceptionDisplayName);
                //}
                //if (this.InsertMapper)
                //{
                //    this.entityGenerateService.InsertMapper();
                //}
                //if (this.InsertEfCoreEntityConfig)
                //{
                //    this.entityGenerateService.InsertEfcoreEntityConfig();
                //}
                if (this.Format)
                {
                    this.reactGenerateService.Format();
                }

            }, this.WhenAnyValue(i => i.EntityPath).Select(i => !string.IsNullOrWhiteSpace(i)));
        }
    }
}
