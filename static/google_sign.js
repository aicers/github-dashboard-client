function renderButton() {
    gapi.signin2.render('my-signin2', {
        'scope': 'profile email',
        'width': 240,
        'height': 50,
        'longtitle': true,
        'theme': 'dark',
        'onsuccess': onSuccess,
        'onfailure': onFailure
    });
}
function onSuccess(googleUser) {
    let btn = document.getElementById("my-signin2");
    const event = new CustomEvent("onsuccess", { detail: googleUser.getBasicProfile().getEmail() });
    btn.dispatchEvent(event);
}
function onFailure(error) {
    console.log(error);
}
