use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 7 5 5-5 5V7" ></ path > < path d = "m21 7-5 5 5 5V7" ></ path > < path d = "M12 20v2" ></ path > < path d = "M12 14v2" ></ path > < path d = "M12 8v2" ></ path > < path d = "M12 2v2" ></ path > < / > } } pub const LUCIDE_FLIP_HORIZONTAL_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;