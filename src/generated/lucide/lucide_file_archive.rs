use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v18" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle r = "2" cy = "20" cx = "10" ></ circle > < path d = "M10 7V6" ></ path > < path d = "M10 12v-1" ></ path > < path d = "M10 18v-2" ></ path > < / > } } pub const LucideFileArchive : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;