use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M4.268 21a2 2 0 0 0 1.727 1H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" ></ path > < path d = "m9 18-1.5-1.5" ></ path > < circle cy = "14" cx = "5" r = "3" ></ circle > < / > } } pub const LUCIDE_FILE_SEARCH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;