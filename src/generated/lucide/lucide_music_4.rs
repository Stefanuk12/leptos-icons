use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle cx = "6" cy = "18" r = "3" ></ circle > < circle cy = "16" cx = "18" r = "3" ></ circle > < / > } } pub const LucideMusic4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;