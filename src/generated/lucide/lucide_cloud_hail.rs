use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M16 14v2" ></ path > < path d = "M8 14v2" ></ path > < path d = "M16 20h.01" ></ path > < path d = "M8 20h.01" ></ path > < path d = "M12 16v2" ></ path > < path d = "M12 22h.01" ></ path > < / > } } pub const LUCIDE_CLOUD_HAIL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2")] } ;