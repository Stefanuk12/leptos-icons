use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" height = "20" rx = "7" y = "2" width = "14" ></ rect > < path d = "M12 6v4" ></ path > < / > } } pub const LUCIDE_MOUSE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;