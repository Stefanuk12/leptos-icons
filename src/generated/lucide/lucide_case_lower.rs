use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9v6" ></ path > < circle cy = "12" r = "3" cx = "17" ></ circle > < path d = "M14 7v8" ></ path > < / > } } pub const LucideCaseLower : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;