use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" width = "7" x = "3" rx = "1" ></ rect > < rect x = "14" rx = "1" width = "7" height = "7" y = "3" ></ rect > < rect rx = "1" height = "7" y = "14" x = "14" width = "7" ></ rect > < rect height = "7" x = "3" width = "7" y = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24")] } ;