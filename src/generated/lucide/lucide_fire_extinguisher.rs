use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 6.5V3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3.5" ></ path > < path d = "M9 18h8" ></ path > < path d = "M18 3h-3" ></ path > < path d = "M11 3a6 6 0 0 0-6 6v11" ></ path > < path d = "M5 13h4" ></ path > < path d = "M17 10a4 4 0 0 0-8 0v10a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2Z" ></ path > < / > } } pub const LUCIDE_FIRE_EXTINGUISHER : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;