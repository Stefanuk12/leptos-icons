use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" width = "14" height = "6" rx = "2" x = "5" ></ rect > < rect x = "7" height = "6" y = "6" rx = "2" width = "10" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;