use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "3" height = "7" x = "3" width = "7" ></ rect > < rect height = "7" width = "7" rx = "1" y = "3" x = "14" ></ rect > < rect width = "7" height = "7" y = "14" rx = "1" x = "14" ></ rect > < rect width = "7" x = "3" rx = "1" height = "7" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;