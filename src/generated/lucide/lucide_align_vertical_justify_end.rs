use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "6" y = "12" width = "14" x = "5" ></ rect > < rect y = "2" x = "7" width = "10" height = "6" rx = "2" ></ rect > < path d = "M2 22h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;