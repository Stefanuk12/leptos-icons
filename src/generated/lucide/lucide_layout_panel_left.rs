use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "7" x = "3" rx = "1" height = "18" ></ rect > < rect y = "3" rx = "1" width = "7" x = "14" height = "7" ></ rect > < rect x = "14" rx = "1" height = "7" y = "14" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;