use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" height = "9" rx = "1" width = "7" ></ rect > < rect y = "3" height = "5" x = "14" rx = "1" width = "7" ></ rect > < rect x = "14" height = "9" rx = "1" width = "7" y = "12" ></ rect > < rect height = "5" x = "3" rx = "1" width = "7" y = "16" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;