use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" rx = "1" width = "7" x = "3" y = "3" ></ rect > < rect width = "7" height = "5" y = "3" rx = "1" x = "14" ></ rect > < rect width = "7" height = "9" x = "14" y = "12" rx = "1" ></ rect > < rect width = "7" height = "5" x = "3" rx = "1" y = "16" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;