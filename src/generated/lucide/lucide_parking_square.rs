use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" y = "3" rx = "2" x = "3" ></ rect > < path d = "M9 17V7h4a3 3 0 0 1 0 6H9" ></ path > < / > } } pub const LucideParkingSquare : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;