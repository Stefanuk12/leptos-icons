use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.66 6H14a2 2 0 0 1 2 2v2.34l1 1L22 8v8" ></ path > < path d = "M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2l10 10Z" ></ path > < line x2 = "22" y1 = "2" y2 = "22" x1 = "2" ></ line > < / > } } pub const LucideVideoOff : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;