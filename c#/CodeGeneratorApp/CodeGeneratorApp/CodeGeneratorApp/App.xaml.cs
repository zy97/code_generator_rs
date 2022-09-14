using CodeGeneratorApp.Modules.ModuleName;
using CodeGeneratorApp.Services;
using CodeGeneratorApp.Services.Interfaces;
using CodeGeneratorApp.Views;
using My.Company;
using Prism.Ioc;
using Prism.Modularity;
using System;
using System.Runtime.InteropServices;
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
                //var custom = new CustomService { is_custom = Optionbool.FromNullable(false) };
                var custom = true;
                var exception = new ExceptionInfo
                {
                    excetpion_name = Marshal.StringToCoTaskMemAnsi("AlreadyExist"),
                    excetpion_code = Marshal.StringToCoTaskMemAnsi("xxxxxx"),
                    excetpion_displayname = Marshal.StringToCoTaskMemAnsi("tttttttttt")
                };
                IntPtr context_ptr = IntPtr.Zero;
                Interop.create(ref context_ptr, "C:\\repo\\Abp.Bom.Blog\\src\\Bom.Blog.Domain\\Tests\\Test.cs");
                Interop.create_dto(context_ptr);
                Interop.create_createorupdatedto(context_ptr);
                Interop.create_ef_repository(context_ptr);
                Interop.create_exception(context_ptr, exception);
                Interop.create_iservice(context_ptr, ref custom);
                Interop.create_manager(context_ptr);
                Interop.create_mancreate_pagedandsortedandfilterresultdtoager(context_ptr);
                Interop.create_service(context_ptr, ref custom);
                Interop.insert_mapper(context_ptr);
                Interop.create_repository_interface(context_ptr);
                Interop.format_all(context_ptr);
                Interop.dispose(ref context_ptr);
                Marshal.FreeCoTaskMem(exception.excetpion_name);
                Marshal.FreeCoTaskMem(exception.excetpion_code);
                Marshal.FreeCoTaskMem(exception.excetpion_displayname);
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
