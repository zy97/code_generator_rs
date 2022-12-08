using System.IO;
using System.Reactive;
using System.Reactive.Linq;
using CodeGeneratorApp.Core.Mvvm;
using CodeGeneratorApp.Modules.EntityGenerate.Services;
using Prism.Regions;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;

namespace CodeGeneratorApp.Modules.ModuleName.ViewModels
{
    public class ViewAViewModel : RegionViewModelBase
    {
        #region entity
        [Reactive]
        public string Namespace { get; set; } = "YourProjectNamespace";
        [Reactive]
        public string IdType { get; set; } = "Guid";
        [Reactive]
        public string EntityName { get; set; } = "Book";
        [Reactive]
        public string EntityDir { get; set; }
        [Reactive]
        public string DtoDir { get; set; }
        [Reactive]
        public string AppServiceDir { get; set; }
        [Reactive]
        public string IAppServiceDir { get; set; }
        [Reactive]
        public bool IsCustomService { get; set; } = false;
        [Reactive]
        public string EfCoreRepositoryDir { get; set; }
        [Reactive]
        public string IRepositoryDir { get; set; }
        [Reactive]
        public string DomainServiceDir { get; set; }
        [Reactive]
        public string ExceptionName { get; set; } = "BookNotFound";
        [Reactive]
        public string ExceptionDir { get; set; }




        [Reactive]
        public bool InsertMapper { get; set; }
        [Reactive]
        public bool InsertEfCoreEntityConfig { get; set; }
        [Reactive]
        public bool Format { get; set; }
        #endregion
        private readonly IEntityGeneratorService entityGenerateService;

        [Reactive]
        public string EntityPath { get; set; }
        [Reactive]
        public string Title { get; set; } = "Abp 服务实体生成";

        public ReactiveCommand<Unit, Unit> Generate { get; set; }
        public ReactiveCommand<Unit, Unit> GenerateEntity { get; set; }



        public ViewAViewModel(IRegionManager regionManager, IEntityGeneratorService entityGenerateService) :
            base(regionManager)
        {
            GenerateEntity = ReactiveCommand.Create(() =>
            {
                if (CheckAndCreateDir(EntityDir))
                    this.entityGenerateService.CreateEntity(Namespace, IdType, EntityName, EntityDir);
            });
            Generate = ReactiveCommand.Create(() =>
            {
                this.entityGenerateService.SetEntity(EntityPath);
                if (CheckAndCreateDir(DtoDir))
                {
                    this.entityGenerateService.CreateDto(DtoDir);
                    this.entityGenerateService.CreateCreateOrUpdateDto(DtoDir);
                    this.entityGenerateService.CreatePagedAndSortedAndFilterResultDto(DtoDir);
                }
                if (CheckAndCreateDir(AppServiceDir))
                {
                    this.entityGenerateService.CreateService(IsCustomService, AppServiceDir);
                }
                if (CheckAndCreateDir(IAppServiceDir))
                {
                    this.entityGenerateService.CreateIService(IsCustomService, IAppServiceDir);
                }
                if (CheckAndCreateDir(IRepositoryDir))
                {
                    this.entityGenerateService.CreateRepositoryInterface(IRepositoryDir);
                }
                if (CheckAndCreateDir(EfCoreRepositoryDir))
                {
                    this.entityGenerateService.CreateEfRepository(EfCoreRepositoryDir);
                }
                if (CheckAndCreateDir(DomainServiceDir))
                {
                    this.entityGenerateService.CreateManager(DomainServiceDir);
                }

                if (CheckAndCreateDir(ExceptionDir))
                {
                    this.entityGenerateService.CreateException(ExceptionName, ExceptionDir);
                }
                if (this.InsertMapper)
                {
                    this.entityGenerateService.InsertMapper("");
                }
                if (this.InsertEfCoreEntityConfig)
                {
                    this.entityGenerateService.InsertEfcoreEntityConfig("");
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
        private bool CheckAndCreateDir(string dir)
        {
            if (string.IsNullOrWhiteSpace(dir))
                return false;
            if (Directory.Exists(dir))
                return true;
            Directory.CreateDirectory(dir);
            return true;
        }
    }
}
