use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "7" height = "7" y = "3" rx = "1" ></ rect > < rect height = "7" x = "14" y = "3" width = "7" rx = "1" ></ rect > < rect y = "14" x = "14" rx = "1" height = "7" width = "7" ></ rect > < rect height = "7" width = "7" x = "3" rx = "1" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;