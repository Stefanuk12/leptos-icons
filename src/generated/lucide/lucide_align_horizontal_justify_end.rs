use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" x = "2" height = "14" rx = "2" width = "6" ></ rect > < rect x = "12" width = "6" height = "10" y = "7" rx = "2" ></ rect > < path d = "M22 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;