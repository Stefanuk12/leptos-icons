use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" y = "3" width = "18" height = "18" ></ rect > < path d = "M9 3v18" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;