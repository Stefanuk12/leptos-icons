use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" x = "3" height = "9" y = "3" rx = "1" ></ rect > < rect y = "3" x = "14" rx = "1" width = "7" height = "5" ></ rect > < rect x = "14" y = "12" width = "7" height = "9" rx = "1" ></ rect > < rect x = "3" height = "5" rx = "1" y = "16" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;