use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" width = "6" y = "6" rx = "2" height = "16" ></ rect > < rect width = "6" x = "14" y = "6" rx = "2" height = "9" ></ rect > < path d = "M22 2H2" ></ path > < / > } } pub const LUCIDE_ALIGN_START_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;