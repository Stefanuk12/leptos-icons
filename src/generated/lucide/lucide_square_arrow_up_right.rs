use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" rx = "2" width = "18" x = "3" ></ rect > < path d = "M8 8h8v8" ></ path > < path d = "m8 16 8-8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_RIGHT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor")] } ;