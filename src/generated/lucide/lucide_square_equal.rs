use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" height = "18" x = "3" width = "18" ></ rect > < path d = "M7 10h10" ></ path > < path d = "M7 14h10" ></ path > < / > } } pub const LUCIDE_SQUARE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;