use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" y = "3" height = "9" x = "3" rx = "1" ></ rect > < rect height = "5" rx = "1" y = "3" x = "14" width = "7" ></ rect > < rect x = "14" y = "12" height = "9" rx = "1" width = "7" ></ rect > < rect x = "3" y = "16" rx = "1" width = "7" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;