use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" height = "18" x = "3" rx = "2" ></ rect > < path d = "M8 8h8v8" ></ path > < path d = "m8 16 8-8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP_RIGHT : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round")] } ;