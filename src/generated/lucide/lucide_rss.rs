use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 11a9 9 0 0 1 9 9" ></ path > < path d = "M4 4a16 16 0 0 1 16 16" ></ path > < circle cy = "19" r = "1" cx = "5" ></ circle > < / > } } pub const LUCIDE_RSS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;