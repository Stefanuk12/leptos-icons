use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" y = "3" width = "18" x = "3" ></ rect > < path d = "M15 3v18" ></ path > < path d = "m8 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_CLOSE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;