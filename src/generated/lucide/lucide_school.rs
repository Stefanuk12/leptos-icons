use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 22v-4a2 2 0 1 0-4 0v4" ></ path > < path d = "m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2" ></ path > < path d = "M18 5v17" ></ path > < path d = "m4 6 8-4 8 4" ></ path > < path d = "M6 5v17" ></ path > < circle cy = "9" cx = "12" r = "2" ></ circle > < / > } } pub const LUCIDE_SCHOOL : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;