use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" x = "5" width = "14" rx = "7" y = "2" ></ rect > < path d = "M12 6v4" ></ path > < / > } } pub const LUCIDE_MOUSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;