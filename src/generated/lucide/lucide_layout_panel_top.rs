use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "1" height = "7" width = "18" x = "3" ></ rect > < rect x = "3" height = "7" rx = "1" width = "7" y = "14" ></ rect > < rect width = "7" y = "14" rx = "1" x = "14" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;