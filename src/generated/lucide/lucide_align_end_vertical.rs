use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" x = "2" height = "6" y = "4" rx = "2" ></ rect > < rect y = "14" width = "9" rx = "2" x = "9" height = "6" ></ rect > < path d = "M22 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;