use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "7" rx = "1" x = "3" y = "3" ></ rect > < rect y = "3" rx = "1" width = "7" height = "7" x = "14" ></ rect > < rect x = "14" rx = "1" width = "7" y = "14" height = "7" ></ rect > < rect height = "7" x = "3" y = "14" width = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;