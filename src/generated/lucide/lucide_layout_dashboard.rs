use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "9" y = "3" rx = "1" x = "3" ></ rect > < rect height = "5" width = "7" x = "14" y = "3" rx = "1" ></ rect > < rect width = "7" height = "9" y = "12" x = "14" rx = "1" ></ rect > < rect height = "5" x = "3" width = "7" y = "16" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;