//https://www.petercollingridge.co.uk/tutorials/svg/interactive/dragging/
console.clear();
const svg = document.querySelector("svg");
const svgns = "http://www.w3.org/2000/svg";
var selectedElement = false;

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
      if (starterPosition[i][j] != '.'){
        piece = document.createElementNS(svgns, "image");
        piece_href = getPieceImageSource(starterPosition[i][j]);
        gsap.set(piece, {
              attr: { 
                  x: j * width  + 15 , 
                  y: i * height + 15,
                  href: piece_href, 
                  height: 50, 
                  width: 50,
                  class: "draggable",
                }
        });
        piece.addEventListener('mousedown', startDrag);
        piece.addEventListener('mousemove', drag);
        piece.addEventListener('mouseup', endDrag);
        piece.addEventListener('mouseleave', endDrag);
        piece.addEventListener('mouseover', mouseOver);
        newRect.addEventListener('mouseover', mouseOver);

        svg.appendChild(newRect);
        svg.appendChild(piece);
        
      } else {
        newRect.addEventListener('mouseover', mouseOver);
        svg.appendChild(newRect);
      }
    }
  }
  getValidMoves();

 
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
            var json = JSON.parse(xhr.responseText);
            console.log(json);
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
    if (evt.target.classList.contains('draggable')) {
        selectedElement = evt.target;
    }
}
function drag(evt) {

}
function endDrag(evt) {
    
}
function mouseOver(evt) {
    console.log(evt.target.id)
}

// let black_rook1 = document.createElementNS(svgns, "image");
// gsap.set(black_rook1, {
//     attr: { x: 150, y: 150,href: "piece_images/black_rook.png", height:"50", width:"50"}
//     });
//     svg.appendChild(black_rook1);

//   let newLine = document.createElementNS(svgns, "line");
//   svg.appendChild(newLine);
//   gsap.set(newLine, {
//     attr: { x1: 0, x2: 100, y1: 100, y2: 100  }
//   });

//   let newRect = document.createElementNS(svgns, "rect");
//   // set attributes of new rectangle
//   gsap.set(newRect, {
//     attr: { x: 150, y: 150, width: 100, height: 100, fill: "#5cceee" }
//   });
  
//   let newCirc = document.createElementNS(svgns, "circle");
//   gsap.set(newCirc, {
//     attr: {
//       cy: 10,
//       cx: 20,
//       r: 10,
//       fill: "red",
//       id: "ed",
//       player: "red"
//     },
//    });
//    svg.appendChild(newCirc);
//    document.getElementById("ed").setAttribute("player","white");
//   // append the new rectangle to the svg

//   newRect.addEventListener("mouseover", create_it);
//   newRect.addEventListener("mouseleave", remove_it);
//   svg.appendChild(newRect);

//   function remove_it() {
//     let circle = document.getElementById("ed");
//     console.log(circle.getAttribute("player"))
//     circle.parentNode.removeChild(circle);
//   }

//   function create_it() {
//     let newCirc = document.createElementNS(svgns, "circle");
//   gsap.set(newCirc, {
//     attr: {
//       cy: 100,
//       cx: 20,
//       r: 10,
//       fill: "red",
//       id: "eddie"
//     },
//    });
//    svg.appendChild(newCirc);
//   }