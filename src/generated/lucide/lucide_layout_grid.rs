use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" rx = "1" x = "3" width = "7" ></ rect > < rect rx = "1" x = "14" width = "7" height = "7" y = "3" ></ rect > < rect width = "7" y = "14" rx = "1" height = "7" x = "14" ></ rect > < rect rx = "1" height = "7" y = "14" width = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;