using CodeGeneratorApp.Core;
using CodeGeneratorApp.Modules.ModuleName.Views;
using Prism.Ioc;
using Prism.Modularity;
using Prism.Regions;

namespace CodeGeneratorApp.Modules.ModuleName
{
    public class EntityGenerateModule : IModule
    {
        private readonly IRegionManager _regionManager;

        public EntityGenerateModule(IRegionManager regionManager)
        {
            _regionManager = regionManager;
        }

        public void OnInitialized(IContainerProvider containerProvider)
        {
            _regionManager.RequestNavigate(RegionNames.ContentRegion, "ViewA");
        }

        public void RegisterTypes(IContainerRegistry containerRegistry)
        {
            containerRegistry.RegisterForNavigation<ViewA>();
        }
    }
}