use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" width = "6" y = "2" rx = "2" height = "16" ></ rect > < rect x = "14" height = "9" width = "6" y = "9" rx = "2" ></ rect > < path d = "M22 22H2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;