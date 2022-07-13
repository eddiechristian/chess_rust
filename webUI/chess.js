//https://www.petercollingridge.co.uk/tutorials/svg/interactive/dragging/
console.clear();
const svg = document.querySelector("svg");
const svgns = "http://www.w3.org/2000/svg";
var selectedElement = false;
var values_map = {};

let columns = 8;
let rows = 8;
let counter = 0;
let width =80;;
let height =80;

const colorArray = ["#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468",
                    "#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B"];

starterPosition = [['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
['.', '.', '.', '.', '.', '.', '.', '.'],
['.', '.', '.', '.', '.', '.', '.', '.'],
['.', '.', '.', '.', '.', '.', '.', '.'],
['.', '.', '.', '.', '.', '.', '.', '.'],
['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r']];

for (let i = 0; i < rows; i++) {
    for (let j = 0; j < columns; j++) {
      square_id = getSquareId(j,i);
      counter++;
      let newRect = document.createElementNS(svgns, "rect");
      let green_circle = document.createElementNS(svgns, "circle");
      gsap.set(newRect, {
        attr: {
          x: j * width,
          y: i * height,
          width: width,
          height: height,
          fill: colorArray[(counter-1) % colorArray.length],
          id: square_id,
        }
      });
      gsap.set(green_circle, {
        attr: {
          cx: j * width + 40,
          cy: i * height + 40,
          r: 9,
          id:  "cir" + square_id,
          class: "valid_moves"
        },
      });
      if (starterPosition[i][j] != '.'){
        piece = document.createElementNS(svgns, "image");
        piece_href = getPieceImageSource(starterPosition[i][j]);
        gsap.set(piece, {
              attr: { 
                  x: j * width  + 15 , 
                  y: i * height + 15,
                  href: piece_href, 
                  height: 45, 
                  width: 45,
                  class: "draggable",
                }
        });
        newRect.addEventListener('mousedown', startDrag);
        newRect.addEventListener('mousemove', drag);
        newRect.addEventListener('mouseup', endDrag);
        newRect.addEventListener('mouseleave', endDrag);
        newRect.addEventListener('mouseover', mouseOver);

        svg.appendChild(newRect);
        svg.appendChild(piece);
        svg.appendChild(green_circle);
        
      } else {
        newRect.addEventListener('mousedown', startDrag);
        newRect.addEventListener('mousemove', drag);
        newRect.addEventListener('mouseup', endDrag);
        newRect.addEventListener('mouseleave', endDrag);
        newRect.addEventListener('mouseover', mouseOver);
        svg.appendChild(newRect);
        svg.appendChild(green_circle);
      }
    }
  }
    
    let green_cicle = document.createElementNS(svgns, "circle");
   
  
    
  getValidMoves(); //only call when turn changes
 
  // async function getValidMoves() {
  //   fetch("http://localhost:9090/chess")
  // .then(r =>  r.json().then(data => ({status: r.status, body: data})))
  // .then(obj => console.log(obj));
  //  }


   async function getValidMoves() {
    var xhr = new XMLHttpRequest();
    var url = "http://localhost:9090/valid_moves";
    xhr.open("POST", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            values_map = JSON.parse(xhr.responseText);
        }
    };
    var data = JSON.stringify({"message": "hey@mail.com"});
    xhr.send(data);
   }

  function getSquareId(col_num, row_num) {
    col = '';
    row = '';
    switch (col_num) {
        case 0: col ='a';break;
        case 1: col ='b';break;
        case 2: col ='c';break;
        case 3: col ='d';break;
        case 4: col ='e';break;
        case 5: col ='f';break;
        case 6: col ='g';break;
        case 7: col ='h';break;
    }
    switch (row_num) {
        case 0: row ='8';break;
        case 1: row ='7';break;
        case 2: row ='6';break;
        case 3: row ='5';break;
        case 4: row ='4';break;
        case 5: row ='3';break;
        case 6: row ='2';break;
        case 7: row ='1';break;
    }
    return col + row;
  }

  function getPieceImageSource(piece) {
    switch (piece) {
        case 'R': return 'piece_images/black_rook.png';
        case 'N': return 'piece_images/black_knight.png';
        case 'B': return 'piece_images/black_bishop.png';
        case 'Q': return 'piece_images/black_queen.png';
        case 'K': return 'piece_images/black_king.png';
        case 'P': return 'piece_images/black_pawn.png';
        case 'r': return 'piece_images/white_rook.png';
        case 'n': return 'piece_images/white_knight.png';
        case 'b': return 'piece_images/white_bishop.png';
        case 'q': return 'piece_images/white_queen.png';
        case 'k': return 'piece_images/white_king.png';
        case 'p': return 'piece_images/white_pawn.png';
    }
}

function startDrag(evt) {
 
}
function drag(evt) {
 
}

function endDrag(evt) {
  elements = document.getElementsByClassName("valid_moves");
  for (var i = 0; i < elements.length; i++) {
    elements[i].style.display = 'none';
  }
}

function show_green_circles(square) {
  let e = document.getElementById("cir" + square);
  e.style.display = 'block';
}


function hide_green_circles(square) {
  let e = document.getElementById("cir" + square);
  e.style.display = 'none';
}
function mouseOver(evt) {
  if (evt.target.id in values_map["moves"]) {
    
    let squares_to_make_visible = values_map["moves"][evt.target.id];
    console.log(squares_to_make_visible)
    squares_to_make_visible.forEach(show_green_circles);
  }
}
