use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" x = "3" width = "18" height = "7" y = "3" ></ rect > < rect x = "3" height = "7" y = "14" width = "7" rx = "1" ></ rect > < rect y = "14" rx = "1" width = "7" x = "14" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;