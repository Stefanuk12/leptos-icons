use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" x = "5" y = "12" rx = "2" ></ rect > < rect width = "10" x = "7" height = "6" rx = "2" y = "2" ></ rect > < path d = "M2 22h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_END : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;