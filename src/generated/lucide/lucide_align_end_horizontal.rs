use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" width = "6" x = "4" rx = "2" y = "2" ></ rect > < rect height = "9" y = "9" width = "6" rx = "2" x = "14" ></ rect > < path d = "M22 22H2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;