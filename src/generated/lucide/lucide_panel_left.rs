use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" y = "3" height = "18" rx = "2" ></ rect > < path d = "M9 3v18" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;