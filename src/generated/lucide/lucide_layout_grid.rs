use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" y = "3" x = "3" rx = "1" height = "7" ></ rect > < rect x = "14" width = "7" height = "7" y = "3" rx = "1" ></ rect > < rect height = "7" y = "14" x = "14" rx = "1" width = "7" ></ rect > < rect rx = "1" y = "14" height = "7" x = "3" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;