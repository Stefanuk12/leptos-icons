use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "5" ry = "2" y = "2" width = "14" height = "20" ></ rect > < path d = "M12.667 8 10 12h4l-2.667 4" ></ path > < / > } } pub const LUCIDE_SMARTPHONE_CHARGING : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;