use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "7" height = "7" x = "3" y = "3" ></ rect > < rect x = "14" width = "7" y = "3" rx = "1" height = "7" ></ rect > < rect y = "14" height = "7" x = "14" rx = "1" width = "7" ></ rect > < rect width = "7" y = "14" x = "3" height = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;