(function($)
{
    $.fn._center = function(self, parent, dimension)
    {
        if(!dimension.vertical && !dimension.horizontal)
            return; //won't do anything anyway
        
        if(parent)
            parent = self.parent();
        else
            parent = window
        
        self.css("position", "absolute");
        
        if(dimension.vertical)
        {
            self.css("top", Math.max(0, (($(parent).height() - $(self).outerHeight()) / 2) + 
                                                        $(parent).scrollTop()) + "px");
        }
        
        if(dimension.horizontal)
        {
            self.css("left", Math.max(0, (($(parent).width() - $(self).outerWidth()) / 2) + 
                                                        $(parent).scrollLeft()) + "px");
        }
        
        return self;
    };
    
    $.fn.center = function(parent, args)
    {
        if(!args)
        {
            args = {horizontal: true, vertical: true};
        }
        
        return this.each(function()
        {
            var obj = $(this);
            
            obj._center(obj, parent, args);
            
            function callback()
            {
                obj._center(obj, parent, args);
            }
            
            callback();
            $(window).resize(callback);
        });
    };
})(jQuery);
