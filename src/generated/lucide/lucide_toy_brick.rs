use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "12" width = "18" y = "8" rx = "1" x = "3" ></ rect > < path d = "M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3" ></ path > < path d = "M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3" ></ path > < / > } } pub const LUCIDE_TOY_BRICK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;