using System.Windows;
using CodeGeneratorApp.Modules.ModuleName;
using CodeGeneratorApp.Modules.ReactGenerate;
using CodeGeneratorApp.Views;
using Prism.Ioc;
using Prism.Modularity;

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
                var custom = false;
                //var exception = new ExceptionInfo
                //{
                //    excetpion_name = Marshal.StringToCoTaskMemAnsi("AlreadyExist"),
                //    excetpion_code = Marshal.StringToCoTaskMemAnsi("xxxxxx"),
                //    excetpion_displayname = Marshal.StringToCoTaskMemAnsi("tttttttttt")
                //};
                //IntPtr context_ptr = IntPtr.Zero;
                //using (var entityGenerator = EntityGenerator.New("C:\\repo\\Abp.Bom.Blog\\src\\Bom.Blog.Domain\\Tests\\Test.cs"))
                //{
                //    entityGenerator.CreateDto();
                //    entityGenerator.CreateCreateorupdatedto();
                //    entityGenerator.CreateEfRepository();
                //    entityGenerator.CreateException("AlreadyExist", "xxxxxx", "tttttttttt");
                //    entityGenerator.CreateIservice(custom);
                //    entityGenerator.CreateManager();
                //    entityGenerator.CreatePagedandsortedandfilterresultdto();
                //    entityGenerator.CreateService(custom);
                //    entityGenerator.InsertMapper();
                //    entityGenerator.CreateRepositoryInterface();
                //    entityGenerator.InsertEfcoreEntityConfig();
                //    entityGenerator.FormatAll();
                //}
                return Container.Resolve<MainWindow>();
            }
            catch (System.Exception ex)
            {

                throw;
            }
        }

        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
        }

        protected override void ConfigureModuleCatalog(IModuleCatalog moduleCatalog)
        {
            moduleCatalog.AddModule<EntityGenerateModule>();
            moduleCatalog.AddModule<ReactGenerateModule>();
        }
    }
}
