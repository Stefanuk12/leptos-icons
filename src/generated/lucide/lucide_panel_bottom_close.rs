use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" x = "3" height = "18" y = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m15 8-3 3-3-3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_CLOSE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;