use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 11a9 9 0 0 1 9 9" ></ path > < path d = "M4 4a16 16 0 0 1 16 16" ></ path > < circle cy = "19" cx = "5" r = "1" ></ circle > < / > } } pub const LUCIDE_RSS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;