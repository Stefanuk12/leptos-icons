use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle r = "6" cx = "8" cy = "16" ></ circle > < path d = "M9.5 17.5 8 16.25V14" ></ path > < / > } } pub const LUCIDE_FILE_CLOCK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;