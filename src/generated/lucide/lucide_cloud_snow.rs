use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M8 15h.01" ></ path > < path d = "M8 19h.01" ></ path > < path d = "M12 17h.01" ></ path > < path d = "M12 21h.01" ></ path > < path d = "M16 15h.01" ></ path > < path d = "M16 19h.01" ></ path > < / > } } pub const LUCIDE_CLOUD_SNOW : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;