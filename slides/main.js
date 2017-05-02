(function () {
    "use strict";

    var slideElements = document.getElementsByClassName("slide");
    var slides = [];
    for (var i = 0; i < slideElements.length; i++) {
        slides.push(slideElements[i].id);
    }

    var currentSlide = 0;
    var step = 4;

    function scroll (direction) {
        currentSlide = currentSlide + (direction);
        if (currentSlide >= 0 && currentSlide < slides.length) {
            console.log(currentSlide, slides[currentSlide - (direction)], slides[currentSlide]);
            var oldElement = document.getElementById(slides[currentSlide - (direction)]);
            var oldTop = oldElement.getBoundingClientRect().top;
            var newElement = document.getElementById(slides[currentSlide]);
            var newTop = newElement.getBoundingClientRect().top;

            console.log(oldTop, newTop);

            oldTop = oldTop + step * (direction);
            window.scrollTo(0, oldTop);

            var scrollInterval = window.setInterval(function () {
                if (oldTop < newTop) {
                    oldTop = oldTop + step * (direction);
                    window.scrollTo(0, oldTop);
                } else {
                    clearInterval(scrollInterval);
                }
            }, step / 4);
        }
    }

    document.addEventListener("keyup", function (event) {
        event.preventDefault();

        if (event.keyCode === 85) {
            scroll(-1);
        } else if (event.keyCode === 68) {
            scroll(1);
        }
    });
})();
