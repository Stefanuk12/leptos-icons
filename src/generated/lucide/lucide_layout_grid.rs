use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "1" height = "7" y = "3" width = "7" ></ rect > < rect rx = "1" height = "7" y = "3" x = "14" width = "7" ></ rect > < rect y = "14" rx = "1" height = "7" x = "14" width = "7" ></ rect > < rect y = "14" width = "7" rx = "1" height = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;