using System;

namespace CodeGeneratorApp.Services.Interfaces
{
    public interface IReactGenerateService : IDisposable
    {
        void CreateApi(string prefix);
        void CreatePage();
        void CreateStore();
        void Format();
        void SetEntity(string entityPath);
    }
}
