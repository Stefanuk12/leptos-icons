use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" rx = "1" x = "3" y = "3" height = "9" ></ rect > < rect width = "7" y = "3" height = "5" rx = "1" x = "14" ></ rect > < rect x = "14" rx = "1" width = "7" y = "12" height = "9" ></ rect > < rect height = "5" rx = "1" y = "16" width = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;