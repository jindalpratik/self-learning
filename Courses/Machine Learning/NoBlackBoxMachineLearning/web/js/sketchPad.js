class SketchPad{
    constructor(container,size=400){
        this.canvas=document.createElement("canvas");
        this.canvas.width = size;
        this.canvas.height = size;
        this.canvas.style=`
            background-color:white;
            box-shadow: 0px 0px 10px 2px black;
        `;
        container.appendChild(this.canvas);
        
        this.ctx=this.canvas.getContext("2d");

        this.#addEventListeners();
    }

    #addEventListeners(){
        this.canvas.onmousedown=(evt)=>{
            const rect=this.canvas.getBoundingClientRect();
            const mouse=[
                evt.clientX-rect.left,
                evt.clientY-rect.top
            ];
            console.log(mouse);
        }
    }
}