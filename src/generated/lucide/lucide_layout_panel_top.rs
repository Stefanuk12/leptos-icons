use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" x = "3" width = "18" y = "3" rx = "1" ></ rect > < rect height = "7" y = "14" rx = "1" width = "7" x = "3" ></ rect > < rect y = "14" rx = "1" height = "7" width = "7" x = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_TOP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;