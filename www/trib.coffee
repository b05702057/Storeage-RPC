me = ""
showing = ""
lclock = 0

seenClock = (c) ->
    if c > lclock
        lclock = c
        console.log("lclock=" + lclock)
    return

listTribs = (data) ->
    ret = JSON.parse(data)
    if ret.err != ""
        appendError(ret.err)
        return
 
    tribs = $("div#tribs")
    tribs.empty()

    if ret.tribs == null || ret.tribs.length == 0
        tribs.append("No Tribble.")
        return

    ul = $("<ul/>")
    ret.tribs.reverse()
    for trib in ret.tribs
        seenClock(trib.clock)
        li = $("<li/>")
        li.append('<span class="author"><a class="author" href="#">@' + 
            trib.user + '</a></span> ')
        li.append('<span class="time">' + trib.time + '</span> ')
        li.append($('<span class="trib" />').text(trib.message))
        li.find("a.author").click((ev)->
            ev.preventDefault()
            name = $(this).text()
            if name.length > 0 && name.indexOf('@') == 0
                name = name.substring(1)
            _showUser(name)
        )
        li.append('<a href="#" class="retrib button">Retribble</a>')
        retrib = li.find("a.retrib")
        retrib.hide()
        li.hover(((ev)->
            if me != ""
                $(this).find("a.retrib").show()
            return
        ), ((ev)->
            $(this).find("a.retrib").hide()
            return
        ))
        retrib.click((->
            msg = trib.message
            who = trib.user
            return (ev) ->
                ev.preventDefault()
                _postRetrib('RT @' + who + ': ' + msg, who)
        )())
        ul.append(li)
    tribs.append(ul)

    return

showHome = (ev) ->
    ev.preventDefault()
    _showHome()
    return

_showHome = ->
    # console.log("show home: " + me)
    $.ajax({
        url: "api/list-home"
        type: "POST"
        data: me
        success: listTribs
        cache: false
    })

    showing = "!home"
    
    $("div#timeline").show()
    $("div#whom").hide()
    $("a#follow").hide()
    $("div#tribs").empty()
    $("h2#title").html("Home of " + me)

    return

showUser = (ev) ->
    ev.preventDefault()
    name = $(this).text()
    _showUser(name)
    return

_showUser = (name) ->
    # console.log("show user: " + name)
    $.ajax({
        url: "api/list-tribs"
        type: "POST"
        data: name
        success: listTribs
        cache: false
    })

    showing = name
    $("h2#title").html(name)

    $("div#tribs").empty()
    $("div#timeline").show()
    $("div#whom").show()
    $("a#follow").show()
    updateFollow()

    return

updateUsers = (data) ->
    ret = JSON.parse(data)
    if ret.err != ""
        appendError(ret.err)
        return

    users = $("#users")
    users.empty()
    if ret.users == null || ret.users.length == 0
        users.append("No user.")
        return

    ul = $("<ul/>")
    for name in ret.users
        ul.append('<li><a href="#">' + 
            name + '</a></li>')
    users.append(ul)
    $("#users li").click(showUser)

    return
    
addUser = ->
    name = $("form#adduser input#username").val()
    if name == ""
        return false

    $("form#adduser input#username").val("")

    console.log("add user", name)
    $.ajax({
        url: "api/add-user"
        type: "POST"
        data: name
        success: updateUsers
        cache: false
    })
    
    return false

listUsers = ->
    $.ajax({
        url: "api/list-users"
        success: updateUsers
        cache: false
    })
    return

appendError = (e) ->
    $("div#errors").show()
    $("div#errors").append('<div class="error">Error: ' +
        e + '</div>')

signIn = (ev) ->
    ev.preventDefault()
    if showing == "" || showing == "!home"
        return

    console.log("sign in as: " + showing)

    me = showing
    $("div#who").show()
    $("div#who h3").html("Signed in as " + me)
    $("div#compose").show()
    $("div#following").show()

    _showHome()
    updateFollow()

    $("div#followings").empty()
    updateFollowing()

    return

signOut = (ev) ->
    console.log("sign out")

    ev.preventDefault()
    me = ""
    $("div#who").hide()
    $("div#compose").hide()
    $("div#following").hide()
    $("a#follow").hide()

    if showing == "!home"
        $("div#timeline").hide()

    return

updateFollowing = ->
    $.ajax({
        url: "api/following"
        type: "POST"
        data: me
        success: _updateFollowing
        cache: false
    })
    return

_updateFollowing = (data) ->
    ret = JSON.parse(data)
    if ret.err != ""
        appendError(ret.err)
        return

    div = $("div#followings")
    div.empty()
    if ret.users == null || ret.users.length == 0
        div.append("Not following anyone.")
        return

    ul = $("<ul/>")
    for name in ret.users
        ul.append('<li><a href="#">' +
            name + '</a></li>')
    div.append(ul)
    $("div#followings li").click(showUser)

    return

hoveringFollow = false

_updateFollow = (data) ->
    ret = JSON.parse(data)
    if ret.err != ""
        appendError(ret.err)
        return

    but = $("a#follow")
    but.unbind("mouseenter mouseleave")
    but.unbind("click")
    if ret.v
        if hoveringFollow
            but.html("Unfollow")
        else
            but.html("Following")
        but.hover(((ev) ->
            $(this).html("Unfollow")
            hoveringFollow = true
            return
        ), ((ev) ->
            $(this).html("Following")
            hoveringFollow = false
            return
        ))
        but.click(unfollow)
    else
        but.html("Follow")
        but.hover(((ev) ->
            hoveringFollow = true
            return
        ), ((ev) ->
            hoveringFollow = false
            return
        ))
        but.click(follow)

    updateFollowing()

    return

follow = (ev) ->
    ev.preventDefault()
    $.ajax({
        url: "api/follow"
        type: "POST"
        data: JSON.stringify({
            who: me
            whom: showing
        })
        success: _updateFollow
        cache: false
    })
    return

unfollow = (ev) ->
    ev.preventDefault()
    $.ajax({
        url: "api/unfollow"
        type: "POST"
        data: JSON.stringify({
            who: me
            whom: showing
        })
        success: _updateFollow
        cache: false
    })
    return

updateFollow = ->
    if me == "" || showing == "!home"
        $("a#follow").hide()
        return

    if me == showing
        but = $("a#follow")
        but.html("Me")
        but.unbind("mouseenter mouseleave")
        but.unbind("click")
        but.hover(((ev) ->
            hoveringFollow = true
            return
        ), ((ev) ->
            hoveringFollow = false
            return
        ))
        but.click(((ev) -> ev.preventDefault()))
        return

    $("a#follow").html("&nbsp;")
    $.ajax({
        url: "api/is-following"
        type: "POST"
        data: JSON.stringify({
            who: me
            whom: showing
        })
        success: _updateFollow
        cache: false
    })
    return

countPostLength = ->
    text = $("form#post textarea").val()
    len = text.length
    left = 140 - len
    $("span#nchar").text(""+left)
    if left < 0
        $("span#nchar").addClass("ncharover")
    else
        $("span#nchar").removeClass("ncharover")
    return

_postRetrib = (msg, at) ->
    $("form#post textarea").val(msg)
    countPostLength()
    _postTrib()
    return

postTrib = (ev) ->
    ev.preventDefault()
    _postTrib()
    return

_postTrib = ->
    text = $("form#post textarea").val()
    len = text.length
    if len == 0
        appendError("empty tweet")
        return
    if len > 140
        appendError("tweet too long")
        return

    $("form#post textarea").val("")
    countPostLength()

    $.ajax({
        url: "api/post"
        type: "POST"
        data: JSON.stringify({
            who: me
            message: text
            clock: lclock
        })
        success: postDone
        cache: false
    })
    return

postDone = (data) ->
    ret = JSON.parse(data)
    if ret.err != ""
        appendError(ret.err)
        return
    
    if showing == ""
        return
    else if showing == "!home"
        _showHome()
    else
        _showUser(showing)
    return

main = ->
    $("form#adduser").submit(addUser)
    $("form#post").submit(postTrib)

    $("div#errors").hide()
    $("div#timeline").hide()

    $("a#signin").click(signIn)
    $("a#home").click(showHome)
    $("a#signout").click(signOut)

    $("form#post textarea").keydown(->
        setTimeout((-> countPostLength()), 1)
    )
    $("form#post textarea").keypress(->
        setTimeout((-> countPostLength()), 1)
    )
    $("form#post textarea").keyup(countPostLength)
    $("form#post textarea").change(countPostLength)

    listUsers()
    return

$(document).ready(main)

