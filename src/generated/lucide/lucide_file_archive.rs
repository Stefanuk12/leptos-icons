use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 12v-1" ></ path > < path d = "M10 18v-2" ></ path > < path d = "M10 7V6" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M15.5 22H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 .274 1.01" ></ path > < circle cy = "20" r = "2" cx = "10" ></ circle > < / > } } pub const LUCIDE_FILE_ARCHIVE : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;