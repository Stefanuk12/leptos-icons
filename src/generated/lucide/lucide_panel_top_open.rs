use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" y = "3" width = "18" x = "3" ></ rect > < path d = "M3 9h18" ></ path > < path d = "m15 14-3 3-3-3" ></ path > < / > } } pub const LUCIDE_PANEL_TOP_OPEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24")] } ;