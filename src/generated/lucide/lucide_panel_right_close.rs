use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" width = "18" y = "3" height = "18" ></ rect > < path d = "M15 3v18" ></ path > < path d = "m8 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_CLOSE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;