(function($)
{
    var results = [];
	
	$("a div.workshopItemTitle").each(function(index, value)
	{
		results.push($(value).parent().attr("href").split("?id=")[1]);
	});
	
	alert(results.join("\n"));
})(jQuery);

//compressed
(function(e){var t=[];e("a div.workshopItemTitle").each(function(n,r){t.push(e(r).parent().attr("href").split("?id=")[1])});alert(t.join("\n"))})(jQuery)
