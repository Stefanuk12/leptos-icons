use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "7" x = "3" height = "18" y = "3" ></ rect > < rect width = "7" height = "7" x = "14" y = "3" rx = "1" ></ rect > < rect rx = "1" x = "14" y = "14" height = "7" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;