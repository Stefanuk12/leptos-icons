use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4" ></ path > < path d = "M7 2h11v4c0 2-2 2-2 4v1" ></ path > < line y1 = "6" y2 = "6" x2 = "18" x1 = "11" ></ line > < line y1 = "2" x2 = "22" y2 = "22" x1 = "2" ></ line > < / > } } pub const LUCIDE_FLASHLIGHT_OFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;