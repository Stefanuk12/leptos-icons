use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" x = "3" height = "18" ></ rect > < path d = "M15 3v18" ></ path > < path d = "m8 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_CLOSE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24")] } ;