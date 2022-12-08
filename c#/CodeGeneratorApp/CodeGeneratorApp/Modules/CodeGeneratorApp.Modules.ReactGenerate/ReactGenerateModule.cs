using CodeGeneratorApp.Modules.ReactGenerate.Services;
using CodeGeneratorApp.Modules.ReactGenerate.Views;
using Prism.Ioc;
using Prism.Modularity;

namespace CodeGeneratorApp.Modules.ReactGenerate
{
    public class ReactGenerateModule : IModule
    {
        public void OnInitialized(IContainerProvider containerProvider)
        {

        }

        public void RegisterTypes(IContainerRegistry containerRegistry)
        {
            containerRegistry.RegisterForNavigation<ViewA>("ViewB");
            containerRegistry.RegisterSingleton<IReactGeneratorService, ReactGeneratorService>();
        }
    }
}