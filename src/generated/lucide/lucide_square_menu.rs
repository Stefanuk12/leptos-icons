use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" height = "18" y = "3" x = "3" ></ rect > < path d = "M7 8h10" ></ path > < path d = "M7 12h10" ></ path > < path d = "M7 16h10" ></ path > < / > } } pub const LUCIDE_SQUARE_MENU : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;