use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" width = "18" y = "3" x = "3" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m16 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_CLOSE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;