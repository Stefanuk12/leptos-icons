use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" width = "7" rx = "1" x = "3" y = "3" ></ rect > < rect y = "3" width = "7" rx = "1" height = "7" x = "14" ></ rect > < rect x = "14" y = "14" rx = "1" height = "7" width = "7" ></ rect > < rect x = "3" height = "7" rx = "1" y = "14" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;