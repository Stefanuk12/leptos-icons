use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" width = "16" height = "6" x = "2" ></ rect > < rect rx = "2" width = "9" x = "9" height = "6" y = "14" ></ rect > < path d = "M22 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;