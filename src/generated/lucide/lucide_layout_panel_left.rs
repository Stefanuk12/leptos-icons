use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "18" x = "3" width = "7" y = "3" ></ rect > < rect rx = "1" y = "3" x = "14" height = "7" width = "7" ></ rect > < rect x = "14" width = "7" height = "7" y = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;