use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 7v4" ></ path > < path d = "M7.998 9.003a5 5 0 1 0 8-.005" ></ path > < rect rx = "2" x = "3" y = "3" height = "18" width = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_POWER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;