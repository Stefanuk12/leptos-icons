use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" x = "3" y = "3" height = "7" rx = "1" ></ rect > < rect width = "7" height = "7" y = "3" x = "14" rx = "1" ></ rect > < rect rx = "1" height = "7" width = "7" y = "14" x = "14" ></ rect > < rect rx = "1" y = "14" width = "7" height = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none")] } ;