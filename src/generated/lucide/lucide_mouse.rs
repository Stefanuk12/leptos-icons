use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" y = "2" height = "20" rx = "7" ></ rect > < path d = "M12 6v4" ></ path > < / > } } pub const LUCIDE_MOUSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;