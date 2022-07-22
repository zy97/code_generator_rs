using System;
using Volo.Abp.Application.Dtos;

namespace Bom.Blog.FriendLinks
{
    public class FriendLinkDto : EntityDto<Guid>
    {
       {{ properties }}
    }
}

{% import "macros.html" as macros %}
<h1>Hello</h1>

{% if my_var %}
    {{ my_var }}
{% else %}
    Sorry, my_var isn't defined.
{% endif %}

{% if not show_all %}
    See more
{% endif %}
{{ "{{ hey }}" }}

{% block content %}
{% endblock content %}

{{ macros::hello_world(greeting="世界") }}