use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v18" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cy = "20" r = "2" cx = "10" ></ circle > < path d = "M10 7V6" ></ path > < path d = "M10 12v-1" ></ path > < path d = "M10 18v-2" ></ path > < / > } } pub const LUCIDE_FILE_ARCHIVE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24")] } ;