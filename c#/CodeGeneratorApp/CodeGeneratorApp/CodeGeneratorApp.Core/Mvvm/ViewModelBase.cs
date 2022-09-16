using Prism.Navigation;
using ReactiveUI;

namespace CodeGeneratorApp.Core.Mvvm
{
    public abstract class ViewModelBase : ReactiveObject, IDestructible
    {
        protected ViewModelBase()
        {

        }

        public virtual void Destroy()
        {

        }
    }
}
