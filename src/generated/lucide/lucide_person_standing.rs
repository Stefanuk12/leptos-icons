use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cy = "5" cx = "12" ></ circle > < path d = "m9 20 3-6 3 6" ></ path > < path d = "m6 8 6 2 6-2" ></ path > < path d = "M12 10v4" ></ path > < / > } } pub const LucidePersonStanding : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;