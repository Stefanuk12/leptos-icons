use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 7v4" ></ path > < path d = "M7.998 9.003a5 5 0 1 0 8-.005" ></ path > < rect y = "3" x = "3" height = "18" width = "18" rx = "2" ></ rect > < / > } } pub const LUCIDE_SQUARE_POWER : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;