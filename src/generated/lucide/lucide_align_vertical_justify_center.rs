use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "6" x = "5" width = "14" y = "16" ></ rect > < rect width = "10" height = "6" x = "7" y = "2" rx = "2" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;