use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" height = "7" x = "3" rx = "1" ></ rect > < rect width = "9" y = "14" rx = "1" height = "7" x = "3" ></ rect > < rect width = "5" x = "16" rx = "1" y = "14" height = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;