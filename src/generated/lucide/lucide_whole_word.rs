use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9v6" ></ path > < circle cx = "17" cy = "12" r = "3" ></ circle > < path d = "M14 7v8" ></ path > < path d = "M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" ></ path > < / > } } pub const LucideWholeWord : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;