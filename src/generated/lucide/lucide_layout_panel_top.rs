use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "7" x = "3" rx = "1" y = "3" ></ rect > < rect width = "7" height = "7" y = "14" x = "3" rx = "1" ></ rect > < rect x = "14" y = "14" width = "7" height = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;