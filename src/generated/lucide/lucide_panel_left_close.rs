use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" width = "18" rx = "2" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m16 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_CLOSE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;