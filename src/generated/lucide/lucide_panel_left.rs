use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "M9 3v18" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;