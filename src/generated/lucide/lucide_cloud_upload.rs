use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 13v8" ></ path > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "m8 17 4-4 4 4" ></ path > < / > } } pub const LUCIDE_CLOUD_UPLOAD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;