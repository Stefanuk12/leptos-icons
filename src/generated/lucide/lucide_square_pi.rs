use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" x = "3" y = "3" width = "18" ></ rect > < path d = "M7 7h10" ></ path > < path d = "M10 7v10" ></ path > < path d = "M16 17a2 2 0 0 1-2-2V7" ></ path > < / > } } pub const LUCIDE_SQUARE_PI : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;