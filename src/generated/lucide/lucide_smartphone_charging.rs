use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "20" rx = "2" x = "5" y = "2" width = "14" ></ rect > < path d = "M12.667 8 10 12h4l-2.667 4" ></ path > < / > } } pub const LUCIDE_SMARTPHONE_CHARGING : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;