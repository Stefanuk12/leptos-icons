use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" width = "7" x = "3" rx = "1" y = "3" ></ rect > < rect width = "7" y = "3" rx = "1" height = "5" x = "14" ></ rect > < rect x = "14" rx = "1" y = "12" width = "7" height = "9" ></ rect > < rect y = "16" width = "7" height = "5" x = "3" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;