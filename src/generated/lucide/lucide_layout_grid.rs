use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "7" x = "3" y = "3" rx = "1" ></ rect > < rect rx = "1" height = "7" x = "14" width = "7" y = "3" ></ rect > < rect y = "14" rx = "1" height = "7" width = "7" x = "14" ></ rect > < rect y = "14" height = "7" width = "7" x = "3" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;