use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "18" x = "3" y = "3" width = "7" ></ rect > < rect height = "7" x = "14" rx = "1" width = "7" y = "3" ></ rect > < rect rx = "1" x = "14" width = "7" height = "7" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_PANEL_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;