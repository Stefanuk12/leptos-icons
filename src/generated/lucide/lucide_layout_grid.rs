use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "7" x = "3" width = "7" y = "3" ></ rect > < rect width = "7" x = "14" rx = "1" height = "7" y = "3" ></ rect > < rect width = "7" x = "14" height = "7" y = "14" rx = "1" ></ rect > < rect x = "3" width = "7" height = "7" y = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;