use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "8" width = "18" height = "12" x = "3" ></ rect > < path d = "M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3" ></ path > < path d = "M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3" ></ path > < / > } } pub const LUCIDE_TOY_BRICK : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;