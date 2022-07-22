using System;
using Volo.Abp.Application.Dtos;

namespace Bom.Blog.FriendLinks
{
    public class FriendLinkDto : EntityDto<Guid>
    {
        public string Title { get; set; }
        public string LinkUrl { get; set; }
    }
    public class AdminFriendLinkDto : EntityDto<Guid>
    {
        public string Title { get; set; }
        public string LinkUrl { get; set; }
    }
}
