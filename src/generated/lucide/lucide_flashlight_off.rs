use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4" ></ path > < path d = "M7 2h11v4c0 2-2 2-2 4v1" ></ path > < line y2 = "6" x1 = "11" x2 = "18" y1 = "6" ></ line > < line x2 = "22" y1 = "2" x1 = "2" y2 = "22" ></ line > < / > } } pub const LUCIDE_FLASHLIGHT_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;