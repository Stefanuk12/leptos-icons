use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle cx = "6" r = "3" cy = "18" ></ circle > < circle cy = "16" r = "3" cx = "18" ></ circle > < / > } } pub const LUCIDE_MUSIC_4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;