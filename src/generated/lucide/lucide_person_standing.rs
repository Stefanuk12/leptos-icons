use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "5" ></ circle > < path d = "m9 20 3-6 3 6" ></ path > < path d = "m6 8 6 2 6-2" ></ path > < path d = "M12 10v4" ></ path > < / > } } pub const LUCIDE_PERSON_STANDING : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;