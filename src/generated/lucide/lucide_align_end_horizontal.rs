use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" height = "16" y = "2" rx = "2" width = "6" ></ rect > < rect rx = "2" width = "6" x = "14" y = "9" height = "9" ></ rect > < path d = "M22 22H2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;