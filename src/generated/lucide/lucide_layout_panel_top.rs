use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" width = "18" height = "7" rx = "1" ></ rect > < rect height = "7" x = "3" width = "7" y = "14" rx = "1" ></ rect > < rect x = "14" width = "7" rx = "1" height = "7" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;