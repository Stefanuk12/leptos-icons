use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m16 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_CLOSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;