use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 7v4" ></ path > < path d = "M7.998 9.003a5 5 0 1 0 8-.005" ></ path > < rect width = "18" height = "18" rx = "2" y = "3" x = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE_POWER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;