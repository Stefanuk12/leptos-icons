use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" rx = "2" height = "14" x = "2" width = "6" ></ rect > < rect rx = "2" width = "6" y = "7" height = "10" x = "12" ></ rect > < path d = "M22 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;