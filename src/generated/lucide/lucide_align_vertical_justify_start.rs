use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" width = "14" x = "5" rx = "2" y = "16" ></ rect > < rect width = "10" x = "7" height = "6" y = "6" rx = "2" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;