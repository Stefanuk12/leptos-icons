use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "14" height = "6" y = "16" x = "5" ></ rect > < rect width = "10" rx = "2" height = "6" y = "2" x = "7" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;