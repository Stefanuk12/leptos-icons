use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "6" x = "5" width = "14" y = "12" ></ rect > < rect y = "2" rx = "2" width = "10" x = "7" height = "6" ></ rect > < path d = "M2 22h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;