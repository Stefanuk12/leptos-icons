use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" rx = "1" height = "7" y = "3" ></ rect > < rect width = "9" x = "3" height = "7" y = "14" rx = "1" ></ rect > < rect y = "14" rx = "1" height = "7" width = "5" x = "16" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;