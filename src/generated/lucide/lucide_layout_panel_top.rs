use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" rx = "1" width = "18" x = "3" ></ rect > < rect y = "14" rx = "1" x = "3" width = "7" height = "7" ></ rect > < rect height = "7" rx = "1" y = "14" x = "14" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none")] } ;