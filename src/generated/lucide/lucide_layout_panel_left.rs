use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "18" x = "3" rx = "1" y = "3" ></ rect > < rect width = "7" height = "7" rx = "1" y = "3" x = "14" ></ rect > < rect x = "14" width = "7" height = "7" y = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;