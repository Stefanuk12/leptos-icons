use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "6" rx = "2" x = "4" height = "16" ></ rect > < rect height = "9" rx = "2" width = "6" x = "14" y = "9" ></ rect > < path d = "M22 22H2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;