document.addEventListener("DOMContentLoaded", function() {
    size_canvas();
});

// Function to size the canvas to the window size
function size_canvas() {
    const canvas = document.getElementById('main_canvas');

    const top_div = document.getElementById('top_div');
    const bottom_div = document.getElementById('bottom_div');

    const style = getComputedStyle(document.body);
    let top_div_height = top_div.clientHeight + (2 * (parseInt(style.marginTop) + parseInt(style.marginBottom)));
    let bottom_div_height = bottom_div.clientHeight + (3 * (parseInt(style.marginTop) + parseInt(style.marginBottom)));


    let width = window.innerWidth - ( parseInt(style.marginLeft) + parseInt(style.marginRight) );
    let height = window.innerHeight - ( parseInt(style.marginTop) + parseInt(style.marginBottom) ) - top_div_height -bottom_div_height;
    let length = Math.min(width, height);

    canvas.width = length;
    canvas.height = length;
}