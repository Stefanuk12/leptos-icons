use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < path d = "m3.2 12.9-.9-.4" ></ path > < path d = "m3.2 15.1-.9.4" ></ path > < path d = "M4.677 21.5a2 2 0 0 0 1.313.5H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2.5" ></ path > < path d = "m4.9 11.2-.4-.9" ></ path > < path d = "m4.9 16.8-.4.9" ></ path > < path d = "m7.5 10.3-.4.9" ></ path > < path d = "m7.5 17.7-.4-.9" ></ path > < path d = "m9.7 12.5-.9.4" ></ path > < path d = "m9.7 15.5-.9-.4" ></ path > < circle cy = "14" r = "3" cx = "6" ></ circle > < / > } } pub const LUCIDE_FILE_COG : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;