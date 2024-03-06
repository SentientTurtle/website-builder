document.getElementById("navigation-hamburger")
    .onclick = function () {
    let button = document.getElementById("navigation-hamburger");

    if (button.classList.contains("hamburger-open")) {
        button.classList.remove("hamburger-open")
        for (let element of document.getElementsByClassName("navigation-dropdown")) {
            element.style.visibility = "hidden";
            element.style.position = "absolute";
        }
    } else {
        button.classList.add("hamburger-open")
        for (let element of document.getElementsByClassName("navigation-dropdown")) {
            element.style.visibility = "visible";
            element.style.position = "relative";
        }
    }
}

for (const button of document.getElementsByClassName("tab-box_button")) {
    button.onclick = function (event) {
        const button = event.currentTarget;
        // Do nothing if clicked on an already-selected button
        if (!button.classList.contains("tab-box_button_selected")) {
            const id = button.id.split(":")[1];

            for (const selected_button of button.parentElement.parentElement.querySelectorAll(".tab-box_button_selected")) {
                selected_button.classList.remove("tab-box_button_selected")
            }

            for (const selected_container of button.parentElement.parentElement.querySelectorAll(".tab-box_container_selected")) {
                selected_container.classList.remove("tab-box_container_selected")
            }

            document.getElementById("tab-box_button:" + id).classList.add("tab-box_button_selected");
            document.getElementById("tab-box_container:" + id).classList.add("tab-box_container_selected");
        }
    }
}

for (const button of document.getElementsByClassName("code-box_fold_button")) {
    button.onclick = function(event) {
        event.currentTarget
            .parentElement
            .parentElement
            .querySelector("code,pre")
            .classList
            .toggle("code-box_fold");
    }
}