use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" x = "3" rx = "2" width = "18" ></ rect > < path d = "M3 9h18" ></ path > < path d = "m9 16 3-3 3 3" ></ path > < / > } } pub const LUCIDE_PANEL_TOP_CLOSE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;