use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "7" cy = "12" ></ circle > < path d = "M10 9v6" ></ path > < circle cy = "12" cx = "17" r = "3" ></ circle > < path d = "M14 7v8" ></ path > < path d = "M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" ></ path > < / > } } pub const LUCIDE_WHOLE_WORD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;