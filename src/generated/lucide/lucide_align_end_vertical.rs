use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "4" x = "2" height = "6" rx = "2" ></ rect > < rect x = "9" y = "14" rx = "2" height = "6" width = "9" ></ rect > < path d = "M22 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2")] } ;