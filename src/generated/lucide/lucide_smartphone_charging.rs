use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" height = "20" y = "2" rx = "2" ry = "2" width = "14" ></ rect > < path d = "M12.667 8 10 12h4l-2.667 4" ></ path > < / > } } pub const LUCIDE_SMARTPHONE_CHARGING : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;