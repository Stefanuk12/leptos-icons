use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cy = "14" r = "3" cx = "5" ></ circle > < path d = "m9 18-1.5-1.5" ></ path > < / > } } pub const LucideFileSearch : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;