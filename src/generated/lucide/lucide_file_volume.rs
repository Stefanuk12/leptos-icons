use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "m7 10-3 2H2v4h2l3 2Z" ></ path > < path d = "M11 11a5 5 0 0 1 0 6" ></ path > < / > } } pub const LUCIDE_FILE_VOLUME : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;