use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" height = "18" width = "18" y = "3" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m16 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_CLOSE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;