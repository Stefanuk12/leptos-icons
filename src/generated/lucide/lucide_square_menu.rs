use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" width = "18" rx = "2" x = "3" ></ rect > < path d = "M7 8h10" ></ path > < path d = "M7 12h10" ></ path > < path d = "M7 16h10" ></ path > < / > } } pub const LUCIDE_SQUARE_MENU : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;