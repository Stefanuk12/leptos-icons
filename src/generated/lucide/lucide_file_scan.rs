use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M16 14a2 2 0 0 0-2 2" ></ path > < path d = "M20 14a2 2 0 0 1 2 2" ></ path > < path d = "M20 22a2 2 0 0 0 2-2" ></ path > < path d = "M16 22a2 2 0 0 1-2-2" ></ path > < / > } } pub const LUCIDE_FILE_SCAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;