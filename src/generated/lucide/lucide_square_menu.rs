use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" rx = "2" height = "18" y = "3" ></ rect > < path d = "M7 8h10" ></ path > < path d = "M7 12h10" ></ path > < path d = "M7 16h10" ></ path > < / > } } pub const LUCIDE_SQUARE_MENU : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;