use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "9" y = "3" width = "7" rx = "1" ></ rect > < rect x = "14" y = "3" height = "5" rx = "1" width = "7" ></ rect > < rect height = "9" y = "12" rx = "1" width = "7" x = "14" ></ rect > < rect x = "3" height = "5" y = "16" width = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;