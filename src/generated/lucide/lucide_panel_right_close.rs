use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" y = "3" rx = "2" x = "3" ></ rect > < path d = "M15 3v18" ></ path > < path d = "m8 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_CLOSE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24")] } ;