use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 11a9 9 0 0 1 9 9" ></ path > < path d = "M4 4a16 16 0 0 1 16 16" ></ path > < circle r = "1" cx = "5" cy = "19" ></ circle > < / > } } pub const LUCIDE_RSS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;