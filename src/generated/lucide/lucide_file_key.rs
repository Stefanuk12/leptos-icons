use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" ></ path > < circle r = "2" cx = "10" cy = "16" ></ circle > < path d = "m16 10-4.5 4.5" ></ path > < path d = "m15 11 1 1" ></ path > < / > } } pub const LUCIDE_FILE_KEY : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;