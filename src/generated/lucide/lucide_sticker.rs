use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z" ></ path > < path d = "M14 3v4a2 2 0 0 0 2 2h4" ></ path > < path d = "M8 13h0" ></ path > < path d = "M16 13h0" ></ path > < path d = "M10 16s.8 1 2 1c1.3 0 2-1 2-1" ></ path > < / > } } pub const LucideSticker : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;