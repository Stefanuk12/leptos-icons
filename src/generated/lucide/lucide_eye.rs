use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" ></ path > < circle cx = "12" cy = "12" r = "3" ></ circle > < / > } } pub const LUCIDE_EYE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;