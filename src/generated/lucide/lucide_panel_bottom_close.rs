use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" width = "18" x = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m15 8-3 3-3-3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_CLOSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2")] } ;