use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" y = "3" x = "3" height = "9" rx = "1" ></ rect > < rect x = "14" width = "7" height = "5" y = "3" rx = "1" ></ rect > < rect width = "7" height = "9" rx = "1" x = "14" y = "12" ></ rect > < rect height = "5" x = "3" width = "7" y = "16" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none")] } ;