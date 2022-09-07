using CodeGeneratorApp.Modules.ModuleName;
using CodeGeneratorApp.Services;
using CodeGeneratorApp.Services.Interfaces;
using CodeGeneratorApp.Views;
using Prism.Ioc;
using Prism.Modularity;
using System.Windows;

namespace CodeGeneratorApp
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App
    {
        protected override Window CreateShell()
        {
            return Container.Resolve<MainWindow>();
        }

        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
            containerRegistry.RegisterSingleton<IMessageService, MessageService>();
        }

        protected override void ConfigureModuleCatalog(IModuleCatalog moduleCatalog)
        {
            moduleCatalog.AddModule<ModuleNameModule>();
        }
    }
}
