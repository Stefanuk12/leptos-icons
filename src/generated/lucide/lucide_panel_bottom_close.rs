use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" rx = "2" y = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m15 8-3 3-3-3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_CLOSE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;