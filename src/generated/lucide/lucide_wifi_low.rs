use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20h.01" ></ path > < path d = "M8.5 16.429a5 5 0 0 1 7 0" ></ path > < / > } } pub const LUCIDE_WIFI_LOW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2")] } ;