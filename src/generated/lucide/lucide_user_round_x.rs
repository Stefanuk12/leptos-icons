use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 11.873-7" ></ path > < circle r = "5" cx = "10" cy = "8" ></ circle > < path d = "m17 17 5 5" ></ path > < path d = "m22 17-5 5" ></ path > < / > } } pub const LucideUserRoundX : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;