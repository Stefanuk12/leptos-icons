use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" height = "6" y = "12" rx = "2" width = "14" ></ rect > < rect y = "2" width = "10" x = "7" height = "6" rx = "2" ></ rect > < path d = "M2 22h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;