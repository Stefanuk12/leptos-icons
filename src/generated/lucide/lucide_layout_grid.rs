use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" width = "7" rx = "1" height = "7" ></ rect > < rect width = "7" y = "3" height = "7" x = "14" rx = "1" ></ rect > < rect width = "7" height = "7" y = "14" rx = "1" x = "14" ></ rect > < rect width = "7" x = "3" height = "7" y = "14" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;