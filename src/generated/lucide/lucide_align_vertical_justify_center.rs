use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" rx = "2" height = "6" y = "16" ></ rect > < rect x = "7" y = "2" height = "6" rx = "2" width = "10" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;