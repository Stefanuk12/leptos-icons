use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" height = "18" width = "18" ></ rect > < path d = "M7 7h10" ></ path > < path d = "M10 7v10" ></ path > < path d = "M16 17a2 2 0 0 1-2-2V7" ></ path > < / > } } pub const LUCIDE_SQUARE_PI : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;