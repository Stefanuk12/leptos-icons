use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "7" y = "3" width = "7" x = "3" ></ rect > < rect y = "3" x = "14" width = "7" height = "7" rx = "1" ></ rect > < rect width = "7" height = "7" x = "14" rx = "1" y = "14" ></ rect > < rect rx = "1" y = "14" x = "3" width = "7" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;