use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" x = "3" rx = "1" width = "7" ></ rect > < rect rx = "1" x = "14" width = "7" y = "3" height = "7" ></ rect > < rect width = "7" height = "7" y = "14" x = "14" rx = "1" ></ rect > < rect height = "7" width = "7" y = "14" rx = "1" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;