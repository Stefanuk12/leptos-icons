use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" x = "2" height = "14" y = "5" rx = "2" ></ rect > < rect width = "6" height = "10" rx = "2" y = "7" x = "16" ></ rect > < path d = "M12 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;