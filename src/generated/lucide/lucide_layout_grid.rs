use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "7" width = "7" rx = "1" ></ rect > < rect height = "7" y = "3" x = "14" rx = "1" width = "7" ></ rect > < rect rx = "1" x = "14" width = "7" y = "14" height = "7" ></ rect > < rect height = "7" width = "7" y = "14" rx = "1" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_GRID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;