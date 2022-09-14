using CodeGeneratorApp.Modules.ModuleName;
using CodeGeneratorApp.Services;
using CodeGeneratorApp.Services.Interfaces;
using CodeGeneratorApp.Views;
using My.Company;
using Prism.Ioc;
using Prism.Modularity;
using System;
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
            try
            {
                IntPtr context_ptr = IntPtr.Zero;
                Interop.create(ref context_ptr, "C:\\repo\\Abp.Bom.Blog\\src\\Bom.Blog.Domain\\Tests\\Test.cs");
                Interop.create_dto(context_ptr);
                Interop.dispose(ref context_ptr);
                //var sdf = Marshal.PtrToStringAnsi(sf.path);
                return Container.Resolve<MainWindow>();
            }
            catch (System.Exception)
            {

                throw;
            }
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
