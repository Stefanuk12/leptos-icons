use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" y = "3" x = "3" rx = "2" ></ rect > < path d = "M7 10h10" ></ path > < path d = "M7 14h10" ></ path > < / > } } pub const LUCIDE_SQUARE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;