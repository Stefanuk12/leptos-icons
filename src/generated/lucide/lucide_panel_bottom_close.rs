use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" rx = "2" y = "3" height = "18" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m15 8-3 3-3-3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_CLOSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;