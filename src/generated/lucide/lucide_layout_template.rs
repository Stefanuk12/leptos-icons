use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "7" y = "3" rx = "1" width = "18" ></ rect > < rect y = "14" rx = "1" width = "9" height = "7" x = "3" ></ rect > < rect width = "5" y = "14" height = "7" x = "16" rx = "1" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;