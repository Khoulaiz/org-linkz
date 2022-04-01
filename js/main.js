$(function() {
    const WARN_IF_OPEN_SO_MANY_LINKS = 10;

    $('#table-of-contents a').smoothScroll();
    $(document).ready(function () {
        $("#search").bind("keydown keypress keyup change", function (event) {
            var search = this.value;
            var caseSensitive = search.match(/[A-Z]/g) != null;
            if (!caseSensitive) search = search.toLowerCase();
            var searchTerms = search.match(/\S+/g);
            var $li = $(".outline-2 li").hide();
            $li.filter(function () {
                return filterLinkz(this, searchTerms, caseSensitive);
            }).show();
            filterTopics();
            if(event.type == 'keypress' && event.which == 13) {
                openVisibleLinks(WARN_IF_OPEN_SO_MANY_LINKS);
            }
        });
        filterTopics();
        //console.log("init done");
    });

    function filterLinkz(element, searchTerms, caseSensitive) {
        if(searchTerms == null) {
            return true;
        }
        var $element = $(element);
        if ($element.find("a[href]").length == 0) {
            return false;
        }
        var linkMatched = searchTerms.every((term) => { return linkMatchesTerm($element, term, caseSensitive) });
        if(linkMatched) {
            return true; 
        } else {
            return false;
        }
    }

    function linkMatchesTerm($link, term, caseSensitive) {
        return linkMatchesWord($link, term, caseSensitive) || linkMatchesTag($link, term, caseSensitive);
    }

    function linkMatchesWord($link, wordTerm, caseSensitive) {
        if(wordTerm.charAt(0) == '#') return false;
        var $hrefs = $link.find("a[href]");
        if (caseSensitive) {
            return $hrefs.first().text().includes(wordTerm) ||
                $hrefs.get(0).href.includes(wordTerm);
        } else {
            return $hrefs.text().toLowerCase().includes(wordTerm) ||
                $hrefs.get(0).href.toLowerCase().includes(wordTerm);
        }
    }

    function linkMatchesTag($link, tagTerm, caseSensitive) {
        if(tagTerm.charAt(0) != '#' || tagTerm.length <= 1) return false;
        var $tags = $link.find('span.tag > span');
        if($tags.length == 0) return false;
        var result = false;
        if (caseSensitive) {
            $tags.each(function() {
                result = result || this.textContent.includes(tagTerm.substring(1));
            });
        } else {
            $tags.each(function() {
                result = result || this.textContent.toLowerCase().includes(tagTerm.substring(1));
            });
        }
        return result;
    }

    function filterTopics() {
        var $topics = $(".outline-2").show();
        $topics.each(function () {
            if ($(this).has("a[href]:visible").length == 0) {
                $(this).hide();
            }
        });
    }

    function openVisibleLinks(warningLimit) {
        var $links = $(".outline-2 a[href]:visible");
        if($links.length > warningLimit) {
            let ok = confirm("Are you sure you want to open " + $links.length + " links in tabs?");
            if(!ok) return;
        }
        $links.each(function () {
            window.open(this.href,'_blank','noreferrer, noopener');
        });
    }
});
