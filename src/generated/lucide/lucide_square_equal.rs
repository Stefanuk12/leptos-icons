use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" width = "18" x = "3" y = "3" ></ rect > < path d = "M7 10h10" ></ path > < path d = "M7 14h10" ></ path > < / > } } pub const LUCIDE_SQUARE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;