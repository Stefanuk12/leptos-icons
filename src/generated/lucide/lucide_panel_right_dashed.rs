use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ></ rect > < path d = "M15 14v1" ></ path > < path d = "M15 19v2" ></ path > < path d = "M15 3v2" ></ path > < path d = "M15 9v1" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_DASHED : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;