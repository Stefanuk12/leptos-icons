use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" height = "16" y = "2" rx = "2" width = "6" ></ rect > < rect rx = "2" y = "9" height = "9" x = "14" width = "6" ></ rect > < path d = "M22 22H2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;