use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0" ></ path > < path d = "M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" ></ path > < path d = "M10 20h4" ></ path > < circle cy = "20" r = "2" cx = "16" ></ circle > < circle r = "2" cy = "20" cx = "8" ></ circle > < / > } } pub const LucideLuggage : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;