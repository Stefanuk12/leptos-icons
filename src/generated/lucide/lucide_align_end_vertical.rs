use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" rx = "2" height = "6" x = "2" y = "4" ></ rect > < rect height = "6" y = "14" x = "9" rx = "2" width = "9" ></ rect > < path d = "M22 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round")] } ;