use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "3" x = "3" width = "7" height = "7" ></ rect > < rect width = "7" x = "14" y = "3" rx = "1" height = "7" ></ rect > < rect width = "7" height = "7" x = "14" y = "14" rx = "1" ></ rect > < rect width = "7" x = "3" height = "7" rx = "1" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;