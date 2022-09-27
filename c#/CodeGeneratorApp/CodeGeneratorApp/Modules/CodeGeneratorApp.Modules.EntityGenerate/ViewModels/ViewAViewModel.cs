using CodeGeneratorApp.Core.Mvvm;
using CodeGeneratorApp.Services.Interfaces;
using Prism.Regions;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using System;
using System.Reactive;
using System.Reactive.Linq;

namespace CodeGeneratorApp.Modules.ModuleName.ViewModels
{
    public class ViewAViewModel : RegionViewModelBase
    {
        #region entity
        [Reactive]
        public bool CreateDto { get; set; }
        [Reactive]
        public bool CreateUpdateOrCreateDto { get; set; }
        [Reactive]
        public bool CreateEfCoreRepository { get; set; }
        [Reactive]
        public bool CreateException { get; set; }
        [Reactive]
        public string ExceptionName { get; set; }
        [Reactive]
        public string ExceptionCode { get; set; }
        [Reactive]
        public string ExceptionDisplayName { get; set; }

        [Reactive]
        public bool CreateService { get; set; }
        [Reactive]
        public bool IsCustomService { get; set; }
        [Reactive]
        public bool CreateManager { get; set; }
        [Reactive]
        public bool CratePageAndFilter { get; set; }
        [Reactive]
        public bool InsertMapper { get; set; }
        [Reactive]
        public bool InsertEfCoreEntityConfig { get; set; }
        [Reactive]
        public bool Format { get; set; }
        #endregion
        private readonly IEntityGenerateService entityGenerateService;

        [Reactive]
        public string Message { get; set; }
        [Reactive]
        public string EntityPath { get; set; }
        [Reactive]
        public string Title { get; set; } = "Abp 服务实体生成";

        public ReactiveCommand<Unit, Unit> SelectEntity { get; set; }
        public ReactiveCommand<Unit, Unit> Generate { get; set; }

        public Interaction<Unit, string> SelectEntityInteraction { get; } = new Interaction<Unit, string>();


        public ViewAViewModel(IRegionManager regionManager, IEntityGenerateService entityGenerateService) :
            base(regionManager)
        {
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
                this.entityGenerateService.SetEntity(EntityPath);
                if (CreateDto)
                    this.entityGenerateService.CreateDto();
                if (CreateUpdateOrCreateDto)
                    this.entityGenerateService.CreateCreateOrUpdateDto();
                if (CratePageAndFilter)
                    this.entityGenerateService.CreatePagedAndSortedAndFilterResultDto();
                if (CreateManager)
                    this.entityGenerateService.CreateManager();
                if (CreateEfCoreRepository)
                {
                    this.entityGenerateService.CreateEfRepository();
                    this.entityGenerateService.CreateRepositoryInterface();
                }
                if (this.CreateService)
                {
                    var isCustom = false;
                    if (this.IsCustomService)
                        isCustom = true;
                    this.entityGenerateService.CreateIService(isCustom);
                    this.entityGenerateService.CreateService(isCustom);
                }
                if (this.CreateException)
                {
                    this.entityGenerateService.CreateException(ExceptionName, ExceptionCode, ExceptionDisplayName);
                }
                if (this.InsertMapper)
                {
                    this.entityGenerateService.InsertMapper();
                }
                if (this.InsertEfCoreEntityConfig)
                {
                    this.entityGenerateService.InsertEfcoreEntityConfig();
                }
                if (this.Format)
                {
                    this.entityGenerateService.FormatAll();
                }

            }, this.WhenAnyValue(i => i.EntityPath).Select(i => !string.IsNullOrWhiteSpace(i)));
            this.entityGenerateService = entityGenerateService;
        }
        public override void OnNavigatedTo(NavigationContext navigationContext)
        {
            //do something
        }
    }
}
