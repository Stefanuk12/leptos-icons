use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "M7 7h10" ></ path > < path d = "M10 7v10" ></ path > < path d = "M16 17a2 2 0 0 1-2-2V7" ></ path > < / > } } pub const LUCIDE_SQUARE_PI : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;