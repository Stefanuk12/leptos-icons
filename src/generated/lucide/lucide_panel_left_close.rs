use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" y = "3" width = "18" height = "18" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m16 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_CLOSE : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;