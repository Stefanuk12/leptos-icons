use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "1" height = "7" width = "7" ></ rect > < rect y = "3" rx = "1" height = "7" width = "7" x = "14" ></ rect > < rect y = "14" width = "7" height = "7" rx = "1" x = "14" ></ rect > < rect rx = "1" width = "7" height = "7" x = "3" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;