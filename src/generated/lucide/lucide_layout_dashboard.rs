use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" y = "3" width = "7" rx = "1" x = "3" ></ rect > < rect x = "14" rx = "1" y = "3" width = "7" height = "5" ></ rect > < rect x = "14" rx = "1" height = "9" y = "12" width = "7" ></ rect > < rect width = "7" y = "16" x = "3" rx = "1" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;