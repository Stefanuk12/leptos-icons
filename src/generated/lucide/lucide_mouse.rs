use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" width = "14" x = "5" y = "2" rx = "7" ></ rect > < path d = "M12 6v4" ></ path > < / > } } pub const LUCIDE_MOUSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;