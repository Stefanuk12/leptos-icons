use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "12" x = "5" height = "6" rx = "2" width = "14" ></ rect > < rect rx = "2" y = "2" x = "7" width = "10" height = "6" ></ rect > < path d = "M2 22h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;