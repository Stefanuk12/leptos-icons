use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" x = "3" height = "7" rx = "1" ></ rect > < rect width = "7" height = "7" x = "3" y = "14" rx = "1" ></ rect > < rect y = "14" x = "14" rx = "1" width = "7" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;