use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "1" width = "7" y = "3" height = "7" ></ rect > < rect height = "7" x = "14" width = "7" y = "3" rx = "1" ></ rect > < rect x = "14" y = "14" width = "7" height = "7" rx = "1" ></ rect > < rect x = "3" width = "7" y = "14" rx = "1" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;