use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 22v-4a2 2 0 1 0-4 0v4" ></ path > < path d = "m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2" ></ path > < path d = "M18 5v17" ></ path > < path d = "m4 6 8-4 8 4" ></ path > < path d = "M6 5v17" ></ path > < circle r = "2" cy = "9" cx = "12" ></ circle > < / > } } pub const LUCIDE_SCHOOL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor")] } ;