use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "9" y = "3" rx = "1" x = "3" ></ rect > < rect width = "7" x = "14" y = "3" rx = "1" height = "5" ></ rect > < rect y = "12" x = "14" height = "9" rx = "1" width = "7" ></ rect > < rect height = "5" y = "16" width = "7" x = "3" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24")] } ;