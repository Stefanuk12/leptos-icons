use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 2 4 4" ></ path > < path d = "m17 7 3-3" ></ path > < path d = "M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5" ></ path > < path d = "m9 11 4 4" ></ path > < path d = "m5 19-3 3" ></ path > < path d = "m14 4 6 6" ></ path > < / > } } pub const LUCIDE_SYRINGE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;