use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v6" ></ path > < path d = "M14 2v4a2 2 0 0 0 2 2h4" ></ path > < circle cy = "16" cx = "4" r = "2" ></ circle > < path d = "m10 10-4.5 4.5" ></ path > < path d = "m9 11 1 1" ></ path > < / > } } pub const LUCIDE_FILE_KEY_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;