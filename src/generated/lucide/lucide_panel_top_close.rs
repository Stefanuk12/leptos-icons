use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" width = "18" y = "3" rx = "2" ></ rect > < path d = "M3 9h18" ></ path > < path d = "m9 16 3-3 3 3" ></ path > < / > } } pub const LUCIDE_PANEL_TOP_CLOSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24")] } ;