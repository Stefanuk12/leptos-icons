use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "9" x = "3" y = "3" rx = "1" ></ rect > < rect x = "14" rx = "1" width = "7" height = "5" y = "3" ></ rect > < rect width = "7" height = "9" y = "12" x = "14" rx = "1" ></ rect > < rect x = "3" height = "5" y = "16" rx = "1" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24")] } ;