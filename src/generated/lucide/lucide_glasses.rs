use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "15" r = "4" cx = "6" ></ circle > < circle cy = "15" cx = "18" r = "4" ></ circle > < path d = "M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2" ></ path > < path d = "M2.5 13 5 7c.7-1.3 1.4-2 3-2" ></ path > < path d = "M21.5 13 19 7c-.7-1.3-1.5-2-3-2" ></ path > < / > } } pub const LUCIDE_GLASSES : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;