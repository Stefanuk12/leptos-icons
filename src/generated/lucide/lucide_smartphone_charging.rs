use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" ry = "2" width = "14" rx = "2" x = "5" height = "20" ></ rect > < path d = "M12.667 8 10 12h4l-2.667 4" ></ path > < / > } } pub const LUCIDE_SMARTPHONE_CHARGING : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;