use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "1" cx = "12" ></ circle > < path d = "m9 20 3-6 3 6" ></ path > < path d = "m6 8 6 2 6-2" ></ path > < path d = "M12 10v4" ></ path > < / > } } pub const LUCIDE_PERSON_STANDING : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;