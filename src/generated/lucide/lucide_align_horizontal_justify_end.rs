use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" height = "14" y = "5" rx = "2" x = "2" ></ rect > < rect x = "12" rx = "2" width = "6" height = "10" y = "7" ></ rect > < path d = "M22 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;