use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" height = "18" y = "3" x = "3" ></ rect > < path d = "M8 8h8v8" ></ path > < path d = "m8 16 8-8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;