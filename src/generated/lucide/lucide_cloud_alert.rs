use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12v4" ></ path > < path d = "M12 20h.01" ></ path > < path d = "M17 18h.5a1 1 0 0 0 0-9h-1.79A7 7 0 1 0 7 17.708" ></ path > < / > } } pub const LUCIDE_CLOUD_ALERT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;