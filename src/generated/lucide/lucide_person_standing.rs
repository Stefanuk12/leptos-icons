use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" cx = "12" r = "1" ></ circle > < path d = "m9 20 3-6 3 6" ></ path > < path d = "m6 8 6 2 6-2" ></ path > < path d = "M12 10v4" ></ path > < / > } } pub const LUCIDE_PERSON_STANDING : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;