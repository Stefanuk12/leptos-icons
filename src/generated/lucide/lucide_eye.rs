use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" ></ path > < circle r = "3" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_EYE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;