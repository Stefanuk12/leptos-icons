use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" x = "3" width = "18" rx = "2" ></ rect > < path d = "M8 12h8" ></ path > < / > } } pub const LUCIDE_SQUARE_MINUS : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;