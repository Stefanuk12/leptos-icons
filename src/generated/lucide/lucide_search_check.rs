use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8 11 2 2 4-4" ></ path > < circle cy = "11" r = "8" cx = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LucideSearchCheck : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;