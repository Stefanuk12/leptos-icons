use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" y = "3" rx = "1" height = "7" x = "3" ></ rect > < rect x = "14" y = "3" width = "7" rx = "1" height = "7" ></ rect > < rect rx = "1" height = "7" width = "7" x = "14" y = "14" ></ rect > < rect y = "14" width = "7" rx = "1" height = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;