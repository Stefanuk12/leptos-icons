use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" height = "14" rx = "2" x = "2" y = "5" ></ rect > < rect x = "12" width = "6" height = "10" y = "7" rx = "2" ></ rect > < path d = "M22 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;