use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" x = "3" width = "18" y = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m9 10 3-3 3 3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_OPEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24")] } ;